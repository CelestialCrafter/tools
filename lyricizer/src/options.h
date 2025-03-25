#pragma once

typedef struct {
	char* host;
	unsigned short port;
} options;

options* load_options();
