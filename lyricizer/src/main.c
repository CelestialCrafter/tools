#include <stdio.h>
#include <stdlib.h>
#include <mpd/client.h>
#include <toml.h>

static const char* OPTIONS_PATH = "options.toml";

typedef struct {
	char* host;
	unsigned short port;
} options;

void parse_options(options* opts, toml_table_t* table) {
	toml_datum_t host = toml_string_in(table, "host");
	if (host.ok) {
		opts->host = host.u.s;
	}

	toml_datum_t port = toml_int_in(table, "port");
	if (port.ok) {
		opts->port = port.u.i;
	}
}

options* load_options() {
	FILE* file = fopen(OPTIONS_PATH, "r");
	if (!file) {
		fprintf(stderr, "options: could not open %s\n", OPTIONS_PATH);
		return NULL;
	}

	char error_buf[256];
	toml_table_t* table = toml_parse_file(file, error_buf, sizeof(error_buf));
	fclose(file);

	if (!table) {
		fprintf(stderr, "options: could not parse %s: %s\n", OPTIONS_PATH, error_buf);
		return NULL;
	}

	options* opts = (options*) calloc(1, sizeof(options));
	if (!opts) {
		fprintf(stderr, "options: out of memory\n");
		return NULL;
	}

	parse_options(opts, table);

	return opts;
}

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

	// cleanup
	mpd_connection_free(conn);

	return 0;
}
