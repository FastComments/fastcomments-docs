An `AuditLog` είναι ένα αντικείμενο που αντιπροσωπεύει ένα ελεγχόμενο συμβάν για ενοικιαστές που έχουν πρόσβαση σε αυτή τη δυνατότητα.

The structure for the AuditLog object is as follows:

[inline-code-attrs-start title = 'Δομή AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Who performed the event. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** The browser that performed the event, when it came from one. **/
    ua?: string;
    /** A hash of the session the event came from, for correlating one person's actions. Never the session itself. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** The id of the object the event was performed on, as opposed to who performed it. **/
    targetId?: string;
    /** A human-readable label for that object, e.g. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` και `targetLabel` περιγράφουν σε τι πραγματοποιήθηκε το συμβάν· `userId` και `username` περιγράφουν ποιος το πραγματοποίησε. Για ενημερώσεις, το `objectDetails.changes` περιέχει έναν χάρτη `{field: {from, to}}` που δείχνει τι άλλαξε πραγματικά.

Το αρχείο ελέγχου (audit log) είναι αμετάβλητο. Επίσης, δεν μπορεί να γραφτεί χειροκίνητα. Η FastComments.com μπορεί μόνο να αποφασίσει πότε θα γράψει στο αρχείο ελέγχου. Ωστόσο, μπορείτε να το διαβάσετε μέσω αυτού του API.

Τα συμβάντα στο αρχείο ελέγχου λήγουν μετά από δύο χρόνια.