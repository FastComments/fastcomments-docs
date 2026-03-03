`UserBadge` είναι ένα αντικείμενο που αντιπροσωπεύει ένα σήμα που έχει εκχωρηθεί σε έναν χρήστη στο σύστημα FastComments.

Τα σήματα μπορούν να εκχωρηθούν σε χρήστες αυτόματα με βάση τη δραστηριότητά τους (όπως αριθμός σχολίων, χρόνος απάντησης, κατάσταση βετεράνου) ή χειροκίνητα από τους διαχειριστές του ιστότοπου.

The structure for the `UserBadge` object is as follows:

[inline-code-attrs-start title = 'Δομή UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Μοναδικός αναγνωριστής για αυτή την ανάθεση σήματος χρήστη */
    id: string
    /** ID του χρήστη στον οποίο εκχωρείται αυτό το σήμα */
    userId: string
    /** ID του ορισμού σήματος από τον κατάλογο σημάτων του tenant */
    badgeId: string
    /** ID του tenant που δημιούργησε/εκχώρησε αυτό το σήμα */
    fromTenantId: string
    /** Πότε δημιουργήθηκε αυτό το σήμα (χιλιοστά του δευτερολέπτου από την epoch) */
    createdAt?: number
    /** Πότε έλαβε αυτό το σήμα ο χρήστης (χιλιοστά του δευτερολέπτου από την epoch) */
    receivedAt?: number
    /** 
     * Ο τύπος του σήματος: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Για σήματα βασισμένα σε κατώφλι, η τιμή κατωφλίου */
    threshold?: number
    /** Το όνομα/ετικέτα του σήματος */
    name?: string
    /** Λεπτομερής περιγραφή του σήματος */
    description?: string
    /** Το κείμενο που εμφανίζεται πάνω στο σήμα */
    displayLabel?: string
    /** URL σε μια εικόνα που εμφανίζεται στο σήμα */
    displaySrc?: string
    /** Χρώμα φόντου για το σήμα (κωδικός hex) */
    backgroundColor?: string
    /** Χρώμα περιγράμματος για το σήμα (κωδικός hex) */
    borderColor?: string
    /** Χρώμα κειμένου για το σήμα (κωδικός hex) */
    textColor?: string
    /** Επιπλέον κλάση CSS για μορφοποίηση */
    cssClass?: string
    /** Για σήματα βετεράνου, το χρονικό κατώφλι σε χιλιοστά του δευτερολέπτου */
    veteranUserThresholdMillis?: number
    /** Εάν αυτό το σήμα εμφανίζεται στα σχόλια του χρήστη */
    displayedOnComments: boolean
    /** Η σειρά εμφάνισης του σήματος */
    order?: number
    /** Εάν οριστεί, αυτό το σήμα εμφανίζεται μόνο στη σελίδα με το αντίστοιχο urlId. Null για παγκόσμια σήματα. */
    urlId?: string | null
}
[inline-code-end]
---