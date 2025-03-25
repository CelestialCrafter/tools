#include <stdio.h>
#include <mpd/client.h>

#include "mpd.h"

struct mpd_connection* init_mpd(char* host, unsigned short port) {
	struct mpd_connection *conn = mpd_connection_new(host, port, 0);
	if (!conn) {
		fprintf(stderr, "mpd: out of memory\n");
		return NULL;
	}

	if (mpd_connection_get_error(conn) != MPD_ERROR_SUCCESS) {
		fprintf(stderr, "mpd: %s\n", mpd_connection_get_error_message(conn));
		mpd_connection_free(conn);
		return NULL;
	}

	return conn;
}

bool fetch_songs(struct mpd_connection* conn) {
	if (!mpd_command_list_begin(conn, false)) {
		fprintf(stderr, "mpd: could not begin list\n");
		return false;
	}

	if (!mpd_send_clear_tag_types(conn)) {
		fprintf(stderr, "mpd: could not clear tag types\n");
		return false;
	}

	enum mpd_tag_type tag_types[3] = {
		MPD_TAG_ARTIST,
		MPD_TAG_ALBUM,
		MPD_TAG_TITLE
	};

	if (!mpd_send_enable_tag_types(conn, tag_types, 3)) {
		fprintf(stderr, "mpd: could not enable tag types\n");
		return false;
	}

	if (!mpd_send_list_all_meta(conn, NULL)) {
		fprintf(stderr, "mpd: could not list all meta\n");
		return false;
	}

	if (!mpd_command_list_end(conn)) {
		fprintf(stderr, "mpd: could not end list\n");
		return false;
	}

	return true;
}

void process_songs(struct mpd_connection* conn, void (*process)(song_info)) {
	struct mpd_song* mpd_song;
	while ((mpd_song = mpd_recv_song(conn)) != NULL) {
		song_info song = {
			.artist = mpd_song_get_tag(mpd_song, MPD_TAG_ARTIST, 0),
			.title = mpd_song_get_tag(mpd_song, MPD_TAG_TITLE, 0),
			.album = mpd_song_get_tag(mpd_song, MPD_TAG_ALBUM, 0)
		};

		if (song.artist && song.title && song.album) {
			process(song);
		} else {
			fprintf(stderr, "mpd: ignoring song due to incomplete metadata\n");
		}

		mpd_song_free(mpd_song);
	}
}

