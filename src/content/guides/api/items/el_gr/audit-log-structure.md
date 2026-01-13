Ένα `AuditLog` είναι ένα αντικείμενο που αντιπροσωπεύει ένα ελεγμένο συμβάν για tenants που έχουν πρόσβαση σε αυτή τη δυνατότητα.

Η δομή του αντικειμένου AuditLog είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    objectDetails?: object;
}
[inline-code-end]

Το audit log είναι αμετάβλητο. Επίσης δεν μπορεί να γραφτεί χειροκίνητα. Μόνο το FastComments.com μπορεί να αποφασίσει πότε θα γράψει στο audit log. Ωστόσο, μπορείτε να το διαβάσετε μέσω αυτού του API.

Τα συμβάντα στο audit log λήγουν μετά από δύο χρόνια.
