# mcap-viewer

`mcap-viewer` demonstrates two different HTTP loading paths:

- a public static MCAP file loaded over HTTP
- a local HTTP-served MCAP file opened with `follow_if_http=true`

## Run the example

```bash
cd /home/rozgo/vertex/leptos-rerun/examples/mcap-viewer
NO_COLOR=false trunk serve --open
```

## Prove `follow_if_http`

Start the helper server with a local MCAP file:

```bash
/home/rozgo/vertex/leptos-rerun/examples/mcap-viewer/scripts/serve_mcap.py \
  --file ~/Downloads/nissan_zala_50_zeg_4_0.mcap
```

That serves the file at:

```text
http://127.0.0.1:4318/recording.mcap
```

The script also prints a ready-to-open viewer URL like:

```text
http://127.0.0.1:8080/?url=http%3A%2F%2F127.0.0.1%3A4318%2Frecording.mcap&follow_if_http=1
```

If the original file lives behind SharePoint or another authenticated download flow, use `--file` after downloading it in the browser. The script also supports `--url`, but that only works for direct public URLs.
