[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Moderasyon Grupları sayfasından oluşturulan kimliklerin bir listesi.

Belirtildiğinde, belirtilen yapılandırma kullanılarak bırakılan yorumlar aynı `moderationGroupIds` kümesini içerecektir.

Eğer bir `Moderator` için bir veya daha fazla [Moderasyon Grubu](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) tanımlanmışsa, o kişi
yalnızca grup(ları) ile ilişkili `Moderate Comments` sayfasındaki yorumları görecektir.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]