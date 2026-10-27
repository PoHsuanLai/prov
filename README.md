# prov

Provenance for the assistant: who acted (`Actor`, `AgentRef`), what an action does (`Effect`), and
the labels that follow data (integrity, confidentiality, source) with the `Witness` that may raise
them. Also the message shape and the OpenTelemetry names the program's daemons share.

Used by docket, almanac, sill, detent and files, each pinned by git rev. Its ids come from
`porter-core`, pinned at the same porter rev as every consumer (porter moves first, then prov).

Moved out of porter on 2026-10-28, unchanged since porter dcc16ada.

Licence: MIT OR Apache-2.0.
