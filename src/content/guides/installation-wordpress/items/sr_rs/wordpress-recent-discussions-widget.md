The Recent Discussions widget displays the pages on your site with the most recent comment activity. It's useful for highlighting threads that are still being added to, so visitors can jump back into active conversations rather than landing on quiet pages.

## Options

- **Title** (optional): Наслов приказан изнад листе. Подразумевано је "Recent Discussions".
- **Count** (optional): Колико дискусија приказати. Опсег од 1 до 50. Подразумевано је 20.

## How to Add It

### Inside a Post or Page

У блоку уређивача, додајте блок **Shortcode** и налепите:

[inline-code-attrs-start title = 'Recent Discussions скратени код'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Атрибут `count` прихвата било коју вредност између 1 и 50.

### In a Sidebar or Footer (Classic Themes)

Идите на **Appearance > Widgets** у вашој WordPress администраторској панелу. Из уметача блокова, потражите "FastComments" и изаберите **FastComments: Recent Discussions**. Превуците га у бочну траку, заглавље или подножје, а затим подесите наслов и број у панелу виџета.

### In a Block Theme (Full Site Editing)

Отворите **Site Editor** под **Appearance > Editor**. Идите до дела шаблона где би требало да се појави виџет, уметните блок **Legacy Widget**, и изаберите **FastComments: Recent Discussions** из падајућег менија.

## Troubleshooting

Виџет се приказује тек након што је FastComments подешавање завршено и ID станара (tenant ID) сачуван. Ако је област виџета празна, завршите подешавање под **FastComments** у WordPress администраторском панелу и поново учитајте страницу.

Ако редослед дискусија изгледа застарело, проверите да ли су основне странице завршиле синхронизацију у FastComments контролној табли. Виџет чита живе податке, тако да ново увезени коментари могу да потраје неко време да се појаве.