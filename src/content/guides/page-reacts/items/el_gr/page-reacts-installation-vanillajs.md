---
Για τα Page Reacts πρέπει να αποφασίσουμε δύο πράγματα:

- Ποιες εικόνες αντιδράσεων να χρησιμοποιηθούν.
- Ένα σύντομο `id` για την ονομασία κάθε αντίδρασης.

Προαιρετικά:

- Μπορείτε επίσης να ορίσετε προαιρετικές ξεχωριστές εικόνες για επιλεγμένες/μη επιλεγμένες αντιδράσεις.
- Μπορείτε να αποφασίσετε αν θέλετε να εμφανίζεται η λίστα χρηστών που αντέδρασαν όταν μετακινείτε το ποντίκι πάνω από μία από τις αντιδράσεις. 

[inline-code-attrs-start title = 'Παράδειγμα κώδικα για το Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.fcConfigs = [{
        target: '#page-reacts-example',
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
    }];
</script>
[inline-code-end]

Η διαμόρφωση για τις frontend βιβλιοθήκες React, Angular κ.λπ. είναι η ίδια.

---