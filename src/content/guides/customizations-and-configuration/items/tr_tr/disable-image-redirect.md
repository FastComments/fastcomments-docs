[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Varsayılan olarak FastComments kullanıcıların resim yüklemesine izin verir. Bir kullanıcı o resme tıkladığında, FastComments varsayılan olarak resmi tam boyutta göstermek için yeni bir sekme açar. Bu bayrağı true olarak ayarlamak bu davranışı devre dışı bırakır:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Eğer resme yapılan tıklamayı kendiniz yakalamayı planlamıyorsanız (bkz. [onImageClicked](#callbacks)), resmin tıklanabilir göründüğü izlenimini kaldırmak için bunun bazı stil düzenlemeleriyle birlikte kullanılmasını öneririz.