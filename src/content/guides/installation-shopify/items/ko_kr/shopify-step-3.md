이제 FastComments를 추가하기 위해 스토어의 테마를 편집하겠습니다.

In the left panel, open `Themes`:

<div class="screenshot white-bg">
    <div class="title">테마 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-2-open-themes.png" alt="테마 열기" />
</div>

Under `Current theme`, select `Actions` and then `Edit code`:

<div class="screenshot white-bg">
    <div class="title">코드 편집</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-3-edit-code.png" alt="코드 편집" />
</div>

This will open up a code editor, with a file browser on the left and the code on the right.

All we need to do is copy a small piece of code that adds FastComments and paste it on the right line in the right file.

In the file browser on the left, scroll down and click `Sections`:

<div class="screenshot white-bg">
    <div class="title">Sections 선택</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-4-sections.png" alt="Sections 선택" />
</div>

Now we're going to scroll down and select `main-article.liquid`:

<div class="screenshot white-bg">
    <div class="title">main-article 선택</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-5-main-article.png" alt="main-article 선택" />
</div>

This will open the *theme template* used to render a single blog article.

You should now see something similar to the following, with `main-article.liquid` selected at the top:

<div class="screenshot white-bg">
    <div class="title">main-article 열림</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-6-main-article-open.png" alt="main-article 열림" />
</div>

---