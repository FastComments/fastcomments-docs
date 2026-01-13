[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments özelleştirilmeye uygun olarak tasarlanmıştır ve widget'larımızın kullandığı yazı tipi de bunun istisnası değildir.

Varsayılan olarak, FastComments mümkün olduğunca geniş bir cihaz yelpazesinde iyi görünmesi için `system font stack` kullanır.

Kendi yazı tiplerinizi tanımlamak için, [Özel CSS dokümantasyonu](/guide-customizations-and-configuration.html#custom-css) sayfasına bakın.

Orada, istediğiniz yazı tiplerini tanımlamanıza olanak sağlayacak özel CSS tanımlama yöntemini bulacaksınız.

#### Yazı Tipini Tanımlama

Yazı tipini geçersiz kılmak için, CSS'inizi `.fast-comments, textarea` seçicilerini kullanarak tanımlamanızı öneririz. Örneğin:

[inline-code-attrs-start title = 'Özel Harici Yazı Tipi Örneği'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]