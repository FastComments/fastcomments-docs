Sayfa Reaksiyonları için iki şeye karar vermeliyiz:

- Hangi reaksiyon görsellerinin kullanılacağı.
- Her reaksiyonu adlandırmak için kısa bir `id`.

İsteğe bağlı olarak:

- Seçili/seçili olmayan reaksiyonlar için isteğe bağlı ayrı görseller tanımlayabilirsiniz.
- Bir reaksiyonun üzerine fareyle gelindiğinde, reaksiyon yapan kullanıcıların listesini gösterip göstermemeye karar verebilirsiniz.

[inline-code-attrs-start title = 'Sayfa Reaksiyonları Kod Örneği'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.FastCommentsUI(document.getElementById('page-reacts-example'), {
        tenantId: 'demo',
        pageReactConfig: {
            showUsers: true,
            reacts: [
                {id: 'droll', src: 'https://docs.fastcomments.com/images/emojis/droll.png'},
                {id: 'he', src: 'https://docs.fastcomments.com/images/emojis/heart-eyes.png'},
                {id: 'laugh', src: 'https://docs.fastcomments.com/images/emojis/laugh.png'},
                {id: 'puke', src: 'https://docs.fastcomments.com/images/emojis/puke.png', selectedSrc: 'https://docs.fastcomments.com/images/emojis/puke-bw.png' },
                {id: 'rofl', src: 'https://docs.fastcomments.com/images/emojis/rofl.png' },
            ]
        }
    });
</script>
[inline-code-end]

React, Angular ve benzeri ön yüz (frontend) kütüphaneleri için yapılandırma aynıdır.

---