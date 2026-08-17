The Top Pages widget displays the most-commented pages on your site. It's useful for surfacing your most-engaging content to new visitors and increasing time on site.

## Опције

- **Наслов** (опционално): Наслов приказан изнад листе. Подразумевано је "Top Pages".

Виџет Топ странице бира сопствени распоред на основу доступних података и не прихвата атрибут count.

## Како га додати

### Унутар поста или странице

У уређивачу блокова, додајте блок **Shortcode** и налепите:

[inline-code-attrs-start title = 'Кратки код за Топ странице'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### У бочној траци или подножју (класичне теме)

Идите на **Appearance > Widgets** у вашој WordPress администраторској конзоли. Из уметача блокова, потражите "FastComments" и изаберите **FastComments: Top Pages**. Превуците га у бочну траку, заглавље или подножје, а затим поставите наслов из панела виџета.

### У блок теми (потпуно уређивање сајта)

Отворите **Site Editor** под **Appearance > Editor**. Идите до дела шаблона где би виџет требао да се појави, уметните блок **Legacy Widget**, и изаберите **FastComments: Top Pages** из падајућег списка.

## Решавање проблема

Виџет се приказује тек након што је FastComments подешавање завршено и tenant ID је сачуван. Ако је област виџета празна, завршите подешавање у оквиру **FastComments** у WordPress администраторском панелу и поново учитајте страницу.