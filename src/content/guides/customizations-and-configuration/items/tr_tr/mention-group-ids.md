[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

**@mentions** otomatik tamamlama için kullanılacak ID'lerin listesi. Kesişen grupları olmadığında kullanıcıların etiketlenmesini engellemek istediğinizde kullanışlıdır.

Belirtildiğinde, `@` karakterini yazdıktan sonra otomatik tamamlama yalnızca diğer gruplardaki kullanıcıları gösterecektir.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]