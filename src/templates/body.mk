{{- for tar in targets }}
{{- call target with tar }}
{{- endfor }}
