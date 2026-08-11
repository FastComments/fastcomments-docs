[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Für größere benutzerdefinierte Styling‑Projekte kann es wünschenswert sein, mit einer sauberen Basis zu beginnen und das Standard‑Styling überhaupt nicht zu verwenden.

Alle Standard‑Stile können entfernt werden, indem der **noStyles**‑Parameter auf true gesetzt wird, wie folgt:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Deaktivieren aller Standardstile'; code-example-end]

Dies kann ohne Code auf der Seite zur Widget‑Anpassung unter „Erweiterte Optionen“ angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Deaktivieren aller Standardstil-Checkbox aktiviert unter Erweiterte Optionen auf der Widget-Anpassungsseite'; title='Deaktivieren aller Standardstile' app-screenshot-end]