Για να επισυνάψετε σχόλια σε κάθε δημοσίευση με τον τρόπο που κάνει η ενσωματωμένη υποστήριξη Disqus του Hugo, προσθέστε μια γραμμή στο template single του θέματος σας (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Το partial αποδίδεται μόνο όταν έχει ρυθμιστεί ένα `tenantId`. Απενεργοποιήστε τα σχόλια σε μια μεμονωμένη σελίδα με front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```