Το `UserBadge` είναι ένα αντικείμενο που αναπαριστά ένα παράσημο (badge) που έχει ανατεθεί σε έναν χρήστη στο σύστημα FastComments.

Τα παράσημα μπορούν να ανατεθούν σε χρήστες αυτόματα με βάση τη δραστηριότητά τους (όπως ο αριθμός σχολίων, ο χρόνος απάντησης, η ιδιότητα βετεράνου) ή χειροκίνητα από τους διαχειριστές του ιστότοπου.

Η δομή του αντικειμένου `UserBadge` έχει ως εξής:

[inline-code-attrs-start title = 'Δομή UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Μοναδικό αναγνωριστικό για αυτή την ανάθεση παράσημου χρήστη */
    id: string
    /** ID του χρήστη στον οποίο έχει ανατεθεί αυτό το παράσημο */
    userId: string
    /** ID του ορισμού του παράσημου από τον κατάλογο παράσημων του tenant */
    badgeId: string
    /** ID του tenant που δημιούργησε/ανέθεσε αυτό το παράσημο */
    fromTenantId: string
    /** Πότε δημιουργήθηκε αυτό το παράσημο (χιλιοστά του δευτερολέπτου από την εποχή epoch) */
    createdAt?: number
    /** Πότε το παρέλαβε ο χρήστης (χιλιοστά του δευτερολέπτου από την εποχή epoch) */
    receivedAt?: number
    /** 
     * Ο τύπος του παράσημου: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Για παράσημα με κατώφλι, η τιμή του κατωφλίου */
    threshold?: number
    /** Το όνομα/ετικέτα του παράσημου */
    name?: string
    /** Λεπτομερής περιγραφή του παράσημου */
    description?: string
    /** Το κείμενο που εμφανίζεται στο παράσημο */
    displayLabel?: string
    /** URL σε εικόνα που εμφανίζεται στο παράσημο */
    displaySrc?: string
    /** Χρώμα φόντου για το παράσημο (κωδικός hex) */
    backgroundColor?: string
    /** Χρώμα περιγράμματος για το παράσημο (κωδικός hex) */
    borderColor?: string
    /** Χρώμα κειμένου για το παράσημο (κωδικός hex) */
    textColor?: string
    /** Επιπλέον κλάση CSS για στυλ */
    cssClass?: string
    /** Για παράσημα βετεράνων, το κατώφλι χρόνου σε χιλιοστά του δευτερολέπτου */
    veteranUserThresholdMillis?: number
    /** Αν αυτό το παράσημο εμφανίζεται στα σχόλια του χρήστη */
    displayedOnComments: boolean
    /** Η σειρά εμφάνισης του παράσημου */
    order?: number
    /** Αν έχει οριστεί, αυτό το παράσημο εμφανίζεται μόνο στη σελίδα με το αντίστοιχο urlId. Null για καθολικά παράσημα. */
    urlId?: string | null
}
[inline-code-end]