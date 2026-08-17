The Recent Comments widget displays the most recent comments posted across your entire site. It's useful in sidebars, footers, or anywhere you want to surface fresh activity to encourage further reading.

## Options

- **Title** (optional): Наслов приказан изнад листе. Подразумевано је „Recent Comments“.
- **Count** (optional): Број коментара за приказ. Опсег од 1 до 50. Подразумевано је 5.

## How to Add It

### Inside a Post or Page

У блочном уређивачу, додајте **Shortcode** блок и налепите:

[inline-code-attrs-start title = 'Последњи коментари shortcode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

`count` атрибут прихвата било коју вредност између 1 и 50.

### In a Sidebar or Footer (Classic Themes)

Идите на **Appearance > Widgets** у вашој WordPress администраторској панелу. Из уметача блокова, потражите „FastComments“ и изаберите **FastComments: Recent Comments**. Превуците га у бочну траку, заглавље или подножје, а затим подесите наслов и број у панелу виџета.

### In a Block Theme (Full Site Editing)

Отворите **Site Editor** преко **Appearance > Editor**. Идите до делова шаблона где би требало да се појави виџет, уметните **Legacy Widget** блок и изаберите **FastComments: Recent Comments** из падајућег менија.

## Troubleshooting

Виџет се приказује тек након што је FastComments подешавање завршено и ID станара (tenant ID) сачуван. Ако је област виџета празна, завршите подешавање у оквиру **FastComments** у WordPress администраторском панелу и поново учитајте страницу.