Για τις Αντιδράσεις Σελίδας πρέπει να αποφασίσουμε δύο πράγματα:

- Ποιες εικόνες αντίδρασης θα χρησιμοποιηθούν.
- Ένα σύντομο `id` για να ονομαστεί κάθε αντίδραση.

Προαιρετικά:

- Μπορείτε επίσης να ορίσετε προαιρετικές ξεχωριστές εικόνες για επιλεγμένες/μη επιλεγμένες αντιδράσεις.
- Μπορείτε να αποφασίσετε αν θέλετε να εμφανίζεται η λίστα χρηστών που αντέδρασαν όταν τοποθετείτε το ποντίκι πάνω σε μία από τις αντιδράσεις. 

[inline-code-attrs-start title = 'Παράδειγμα κώδικα για τις Αντιδράσεις Σελίδας'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Η διαμόρφωση για τις βιβλιοθήκες frontend όπως React, Angular κ.λπ. είναι η ίδια.