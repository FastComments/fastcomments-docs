Το widget Top Pages εμφανίζει μια λίστα με τις σελίδες σας που έχουν τα περισσότερα σχόλια.

Αυτό το widget περιλαμβάνει ελάχιστο προεπιλεγμένο στυλ και έχει σχεδιαστεί ώστε να μπορεί να προσαρμοστεί εύκολα με το δικό σας CSS.

## Widget Structure

Το widget αποδίδεται με την ακόλουθη δομή HTML:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Top Pages Default CSS Reference

Το widget περιλαμβάνει το ακόλουθο ελάχιστο προεπιλεγμένο στυλ:

[inline-code-attrs-start title = 'Προεπιλεγμένο CSS του widget Top Pages'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## Customization Examples

### Add Styling to Links
```css
.fastcomments-top-pages .title-link {
    color: #0066cc !important;
    text-decoration: none !important;
    font-size: 14px !important;
}

.fastcomments-top-pages .title-link:hover {
    text-decoration: underline !important;
}
```

### Add Borders Between Pages
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### Style the Comment Count
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---