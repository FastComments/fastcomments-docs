Widget Recent Discussions prikazuje listu stranica sortiranu po najnovijoj aktivnosti komentara. Sadrži naslov, datume posljednje aktivnosti, broj komentara s ikonama i automatsko otkrivanje tamnog režima.

## Basic Installation

[inline-code-attrs-start title = 'Instalacija widgeta Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuration Options

- **tenantId** (obavezno): Vaš FastComments tenant ID
- **count** (neobavezno): Broj stranica za prikaz. Zadano je `20`, maksimalno `100`
- **hasDarkBackground** (neobavezno): Prisiljava tamni stil prikaza. Ako nije postavljeno, automatski se otkriva iz pozadine stranice

## Widget Structure

Widget se prikazuje sa sljedećom HTML strukturom:

[inline-code-attrs-start title = 'HTML struktura widgeta Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rd2">
    <div class="fc-rd2-heading">Recent Discussions</div>
    <div class="fc-rd2-list">
        <div class="fc-rd2-item">
            <div class="fc-rd2-detail">
                <a class="fc-rd2-title" href="...">Page Title</a>
                <span class="fc-rd2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-rd2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## Default CSS Reference

[inline-code-attrs-start title = 'Podrazumijevani CSS widgeta Recent Discussions'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rd2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rd2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rd2-item:last-child { border-bottom: none; }
.fc-rd2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-rd2-activity { font-size: 11px; color: #999; }
.fc-rd2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## Customization Examples

### Remove the Container Border

[inline-code-attrs-start title = 'Uklonite granicu kontejnera'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

### Custom Link Color

[inline-code-attrs-start title = 'Prilagođena boja linka'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
a.fc-rd2-title:hover {
    color: #e63946 !important;
}
[inline-code-end]

---