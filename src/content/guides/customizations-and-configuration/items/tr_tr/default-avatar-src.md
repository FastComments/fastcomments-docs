[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Bir kullanıcı FastComments ile ilk kez yorum yaptığında avatarlarını <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a> adresinden almaya çalışırız.

Ancak bir avatar bulamazsak veya kullanıcı hesabında hiç avatar ayarlamamışsa, statik bir varsayılan avatar resmi gösteririz.

Kendi statik avatar resminizi belirtmek için *defaultAvatarSrc* ayarını kullanabilirsiniz.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Bu işlem kod olmadan da yapılabilir. Widget özelleştirme sayfasında "Varsayılan Avatar" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Belirli bir kullanıcı için avatarın tanımlanmasının, örneğin SSO ile, kendi bölümünde ele alındığını unutmayın.