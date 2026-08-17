[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Bir liste, [Moderasyon Grupları](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) sayfasından oluşturulan kimlikler.

Belirtilirse, belirtilen yapılandırma kullanılarak bırakılan yorumlar aynı `moderationGroupIds` kümesini içerir.

Bir `Moderator` bir veya daha fazla [Moderasyon Grupları](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) tanımlıysa, yalnızca grup(ler)iyle ilişkili `Moderate Comments` sayfasındaki yorumları görür.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Moderasyon Gruplarını Belirle'; code-example-end]

---