{{- if phony }}
.PHONY: { name }
{{- endif }}

{{- for pre in prerequisites }}
{ name }: { pre }
{{- endfor }}

{{- if rules }}
{ name }:
	{{- for rule in rules }}
	{ rule }
	{{- endfor }}
{{- else }}
{{- if not prerequisites }}
{ name }:
{{- endif}}
{{- endif }}
