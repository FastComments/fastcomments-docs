[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Bir kullanıcı FastComments ile ilk kez yorum yaptığında avatarını <a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a> adresinden almaya çalışacağız.

Ancak, bir avatar bulamazsak ya da kullanıcı hesabında hiç avatar ayarlamazsa, statik bir varsayılan avatar resmi gösteririz.

Kendi statik avatar resminizi belirtmek için *defaultAvatarSrc* ayarını kullanabiliriz.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Varsayılan Avatarı Geçersiz Kıl'; code-example-end]

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Varsayılan Avatar" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Widget özelleştirme sayfasının Varsayılan Avatar bölümü, burada yedek avatar görüntü URL\'sini ayarlarsınız'; title='Varsayılan Avatarı Özelleştirme' app-screenshot-end]

SSO gibi belirli bir kullanıcı için avatar tanımlamanın kendi bölümünde ele alındığını unutmayın.