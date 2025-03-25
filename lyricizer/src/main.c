#include <stdio.h>
#include <mpd/client.h>

#include "options.h"
#include "mpd.h"

void process(song_info song) {
	printf("dbg: %s %s %s\n", song.artist, song.title, song.album);
}

int main() {
	options* opts = load_options();
	if (!opts) {
		fprintf(stderr, "main: could not load options\n");
		return 1;
	}

	printf("main: using options: host %s, port %d\n", opts->host, opts->port);

	struct mpd_connection* conn = init_mpd(opts->host, opts->port);
	if (!conn) {
		fprintf(stderr, "main: could not initialize mpd\n");
		return 1;
	}

	if (!fetch_songs(conn)) {
		fprintf(stderr, "main: could not send track list commands\n");
		return 1;
	}

	process_songs(conn, process);

	// cleanup
	mpd_connection_free(conn);

	return 0;
}

