[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Μια λίστα με ids για χρήση στην αυτόματη συμπλήρωση των **@mentions**. Χρήσιμο όταν θέλετε να αποτρέψετε τη σήμανση χρηστών όταν δεν έχουν κοινές ομάδες.

Όταν καθορίζεται, μόνο χρήστες από άλλες ομάδες θα εμφανίζονται στην αυτόματη συμπλήρωση μετά την πληκτρολόγηση του χαρακτήρα `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---