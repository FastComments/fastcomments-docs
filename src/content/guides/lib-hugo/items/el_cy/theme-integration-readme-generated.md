---
Για να επισυνάψετε σχόλια σε κάθε ανάρτηση με τον τρόπο που το κάνει η ενσωματωμένη υποστήριξη Disqus του Hugo, προσθέστε μία γραμμή στο single template του θέματος σας (`layouts/_default/single.html`):

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

Το partial αποδίδεται μόνο όταν έχει ρυθμιστεί ένα `tenantId`. Απενεργοποιήστε τα σχόλια σε μεμονωμένη σελίδα μέσω του front matter:

```toml
+++
title = "A page with no comments"
comments = false
+++
```
---