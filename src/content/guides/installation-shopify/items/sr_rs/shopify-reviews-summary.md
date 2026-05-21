---
The **FastComments - Reviews Summary** block shows an aggregated star rating and review breakdown for a page. Pair it with the **FastComments** block on product templates for the standard reviews layout: summary up top, review form and reviews below.

### Претпоставка: подесите Ratings & Reviews

The Reviews Summary block displays the rating questions you configured for your store. Set those up first:

1. Отворите апликацију FastComments у вашем Shopify администраторском панелу.
2. Кликните плочицу **Ratings & Reviews Helper** (или отворите [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) директно).
3. Додајте питања на која желите да сваки рецензент одговори (укупна оцена у звездама, "како је пристајало", итд.).

Without questions configured, the summary block has nothing to aggregate.

### Додавање блока

1. Отворите уређивач теме у Shopify-у.
2. Отворите шаблон **Product** (или шаблон странице на којој желите сажетак).
3. Кликните **Add block** близу врха секције странице, изнад места где ће бити блок **FastComments**.
4. Испод **Apps**, изаберите **FastComments - Reviews Summary**.
5. Додајте блок **FastComments** ниже на истој страници ако то већ нисте урадили, како би посетиоци могли да остављају рецензије.
6. Кликните **Save**.

### Подешавања

| Подешавање | Шта ради | Подразумевано |
|---|---|---|
| Tenant ID (optional) | Одредите из којег FastComments tenant-а сажетак преузима податке. Оставите празно да бисте користили tenant који је аутоматски конфигурисан за продавницу. | (blank) |
| Custom URL ID | Одредите идентификатор странице против којег сажетак агрегира податке. Користите ово када се сажетак налази на другој страници од оне на којој је блок FastComments који он одражава. | (auto-detected) |

### Како се сажетак поклапа са рецензијама

The Reviews Summary block uses the same auto-detection logic as the **FastComments** block:

- Шаблон производа: `shopify-product-{product.id}`
- Шаблон блога: `shopify-article-{article.id}`
- Остали шаблони: путања захтева

For a normal product page, the summary and the comment thread share a URL ID automatically, with no configuration needed.

### Савети

- Сажетак је само за читање. Да бисте прикупили рецензије, потребан вам је блок **FastComments** на истој страници.
- Ако промените питања за оцењивање у Ratings & Reviews Helper након прикупљања рецензија, сажетак ће се прерачунати у складу са новим скупом питања.

---