[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

Som standard er kommentarfeltet **før** kommentartråden. Men ved at sætte denne konfigurationsparameter
til true kan vi flytte det til **efter**.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Dette kan tilpasses uden kode, på widgetens tilpasningsside:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; title='Moving The Reply Box to The Bottom' app-screenshot-end]