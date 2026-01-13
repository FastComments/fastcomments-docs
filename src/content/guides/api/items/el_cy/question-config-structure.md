Το FastComments παρέχει έναν τρόπο κατασκευής ερωτήσεων και συγκέντρωσης των αποτελεσμάτων τους. Ένα παράδειγμα ερώτησης (εφεξής `QuestionConfig`)
μπορεί να είναι μια βαθμολογία με αστέρια, ένας ολισθητής ή μια ερώτηση NPS (καθορίζεται μέσω του `type`).

Τα δεδομένα ερωτήσεων μπορούν να συγκεντρωθούν μεμονωμένα, μαζί, με την πάροδο του χρόνου, συνολικά, ανά σελίδα και ούτω καθεξής.

Το πλαίσιο έχει όλες τις δυνατότητες που χρειάζονται για την κατασκευή widgets πλευράς πελάτη (με τον διακομιστή σας μπροστά από αυτό το API), πίνακες ελέγχου διαχείρισης και εργαλεία αναφοράς.

Πρώτα, πρέπει να ορίσουμε ένα `QuestionConfig`. Η δομή είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type QuestionConfigType = 'nps' | 'slider' | 'star' | 'thumbs';

interface QuestionConfig {
    id: string
    tenantId: string
    name: string
    question: string
    helpText?: string
    createdAt: string
    createdBy: string
    /** READONLY - incremented for each new response. **/
    usedCount: number
    /** A date string for when the configuration was last used (result left). **/
    lastUsed?: string
    type: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder: number
}
[inline-code-end]
