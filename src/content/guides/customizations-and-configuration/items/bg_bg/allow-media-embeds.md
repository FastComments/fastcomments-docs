By default FastComments does not allow iframes in comments. When you enable media embeds, commenters can paste the embed code (the `<iframe>` snippet) from trusted providers like YouTube, Vimeo, SoundCloud, and Spotify, and it will render inline in the comment.

По подразбиране FastComments не позволява iframe елементи в коментарите. Когато активирате медийни вграждания, коментаторите могат да поставят кода за вграждане (фрагмента `<iframe>`) от доверени доставчици като YouTube, Vimeo, SoundCloud и Spotify, и той ще се визуализира в реда на коментара.

For security, this is not a client-side widget config flag. It is a server-side setting, validated when each comment is saved, so it cannot be turned on from the page. Only iframes pointing at a built-in list of trusted providers are allowed. Any other iframe is removed.

За сигурност това не е флаг за конфигурация на уиджета от клиентската страна. Това е настройка от сървърната страна, проверявана при запазване на всеки коментар, така че не може да бъде включена от страницата. Само iframe‑ове, които сочат към вградения списък с доверени доставчици, са разрешени. Всички други iframe‑ове се премахват.

This is done without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Настройка за медийни вграждания включена в страницата за персонализиране на уиджета, позволяваща на коментаторите да поставят доверени iframe вграждания'; title='Разрешаване на медийни вграждания' app-screenshot-end]

### Adding Your Own Providers

If you want to allow embeds from a provider that is not on the built-in trusted list, add its hostname in the "Additional Embed Domains" field on the same page. These hostnames are allowed in addition to the built-in providers. Matching is exact, so include the full hostname (for example, player.example.com). Anything you do not list stays blocked.

Ако искате да разрешите вграждания от доставчик, който не е в вградения списък с доверени доставчици, добавете неговото име на хост в полето „Additional Embed Domains“ (Допълнителни домейни за вграждане) на същата страница. Тези имена на хостове се разрешават в допълнение към вградените доставчици. Съответствието е точно, затова включете пълното име на хост (например, player.example.com). Всичко, което не е изброено, остава блокирано.

Both the plain comment box and the WYSIWYG editor support pasting an embed. In the WYSIWYG editor the embed is inserted as a removable block.

Как обикновеното поле за коментари, така и WYSIWYG редакторът поддържат поставяне на вграждане. В WYSIWYG редактора вграждането се вмъква като блок, който може да се премахне.