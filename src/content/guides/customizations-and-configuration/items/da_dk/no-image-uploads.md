[related-parameter-start name = 'noImageUploads'; type = 'boolean'; related-parameter-end]

Som standard tillader FastComments billeduploads. Dette kan deaktiveres ved at sætte noImageUploads-flaget til true.

[code-example-start config = {noImageUploads: true}; linesToHighlight = [6]; title = 'Deaktivering af billeduploads'; code-example-end]

Dette kan tilpasses uden kode på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-image-uploads'; selector = '.disable-image-uploads'; alt='Deaktiver billeduploads afkrydsningsfeltet slået til i widget-tilpasningssidens indstillinger'; title='Deaktivering af billeduploads' app-screenshot-end]