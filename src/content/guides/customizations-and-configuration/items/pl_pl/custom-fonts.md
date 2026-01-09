[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments jest zaprojektowany tak, aby można go było dostosować, a czcionka używana przez nasze widżety nie jest wyjątkiem.

Domyślnie FastComments używa `system font stack`, aby prezentować się możliwie najlepiej na różnych urządzeniach.

Aby zdefiniować własne czcionki, zobacz [Dokumentację niestandardowego CSS](/guide-customizations-and-configuration.html#custom-css).

Tam znajdziesz sposób na zdefiniowanie niestandardowego CSS, co pozwoli ci określić pożądane czcionki.

#### Jak zdefiniować czcionkę

Aby nadpisać czcionkę, zalecamy zdefiniować swój CSS przy użyciu selektorów `.fast-comments, textarea`. Na przykład:

[inline-code-attrs-start title = 'Przykład zewnętrznej niestandardowej czcionki'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---