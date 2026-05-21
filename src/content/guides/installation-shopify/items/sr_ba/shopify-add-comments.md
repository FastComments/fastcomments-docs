---
Блок **FastComments** је главни видгет за коментаре. Додајте га у шаблоне блог постова, шаблоне производа, или било коју другу страницу гдје желите нит дискусије или уживо ћаскање.

### Add the block

1. Отворите уређивач тема Shopify продавнице (**Online Store > Themes > Customize**).
2. Изаберите шаблон на који желите коментаре: **Blog post**, **Product**, или било који други шаблон странице или секције.
3. У секцији гдје желите да се коментари појаве, кликните **Add block**.
4. Испод **Apps**, изаберите **FastComments**.
5. Кликните **Save**.

Блок се појављује одмах. Нема Tenant ID-а за унос; tenant ваше продавнице је аутоматски повезан када инсталирате апликацију.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Замијените FastComments tenant који блок користи за приказивање. Оставите празно да бисте користили tenant који је аутоматски конфигурисан за продавницу. Пронађите ручни tenant ID на fastcomments.com/auth/my-account/api-secret. | (празно) |
| SSO | Аутоматски пријављује посјетиоца као њихов Shopify налог купца прије коментирања. Видите [Аутоматско пријављивање Shopify купаца](/guide-installation-shopify.html#shopify-sso). | Укључено |
| Commenting Style | **Threaded** за унутрашње одговоре и гласање, или **Streaming** за реално-времено ћаскање. | Threaded |
| Custom URL ID | Замијените аутоматски препознат идентификатор странице. Користите ово када желите да двије URL адресе дијеле једну нит коментара. | (аутоматски препознат) |

### How the page identifier is chosen

Each comment thread is keyed by a URL ID. The block picks one automatically:

- **Blog post template:** `shopify-article-{article.id}`, which is stable across slug and title changes.
- **Product template:** `shopify-product-{product.id}`, which is stable across slug and title changes.
- **Other templates:** the request path.

If you set **Custom URL ID**, that value is used instead. Use the same Custom URL ID across multiple blocks (for example, on a localized variant of a product page) to share one comment thread.

### Threaded vs Streaming

**Threaded** је подразумевано. Посјетиоци одговарају једни другима, гласају, а алати за модерацију функционишу очекивано. Најбоље за блог постове и рецензије производа.

**Streaming** уклања нитовање и приказује нове коментаре у реалном времену како су објављени, попут ћаскања. Најбоље за лансирања производа, догађаје уживо и странице заједнице.

### Multiple blocks on the same page

Блок се може додати више пута у исти шаблон. На примјер, резиме оцјена на врху странице производа и FastComments блок на дну. Блокови дијеле исти URL ID, тако да резиме одражава коментаре испод.

### Tips

- Блок се сакрије у претпрегледу уређивача теме са жућим обавјештењем ако не може пронаћи tenant-а. Ако се то појави у вашој живој продавници, поново инсталирајте FastComments апликацију.
- За страницу производа, FastComments блок служи и као виџет за рецензије производа. Упарите га са **FastComments - Reviews Summary** за резиме оцјена звјездица на врху странице.

---