#pragma once
#include <mpd/client.h>

typedef struct {
	const char* artist;
	const char* title;
	const char* album;
} song_info;

struct mpd_connection* init_mpd(char* host, unsigned short port);

bool fetch_songs(struct mpd_connection* conn);

void process_songs(struct mpd_connection* conn, void (*process)(song_info));

