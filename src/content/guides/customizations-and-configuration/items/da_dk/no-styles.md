---
[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

For større tilpassede stylingprojekter kan det være ønskeligt at starte fra bunden og slet ikke bruge standardstylingen.

Al standardstyling kan fjernes ved at sætte **noStyles**-parameteren til true, som følger:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Dette kan tilpasses uden kode, på widgetens tilpasningsside, under Avancerede indstillinger:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]

---