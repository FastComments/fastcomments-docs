[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments er designet til at blive tilpasset, og skrifttypen vores widgets bruger er ingen undtagelse.

Som standard bruger FastComments `system font stack` for at se så godt ud som muligt på en bred vifte af enheder.

For at definere dine egne skrifttyper, se [Dokumentation for brugerdefineret CSS](/guide-customizations-and-configuration.html#custom-css).

Der finder du en måde at definere brugerdefineret CSS på, som gør det muligt at angive de skrifttyper, du ønsker.

#### Sådan defineres skrifttypen

For at tilsidesætte skrifttypen anbefaler vi, at du definerer din CSS ved hjælp af `.fast-comments, textarea` selektorerne. For eksempel:

[inline-code-attrs-start title = 'Eksempel på ekstern brugerdefineret skrifttype'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---