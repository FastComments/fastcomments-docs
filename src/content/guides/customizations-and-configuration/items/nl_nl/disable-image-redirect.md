[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Standaard staat FastComments gebruikers toe afbeeldingen te uploaden. Wanneer een gebruiker op die afbeelding klikt, opent FastComments standaard,
een nieuw tabblad om die afbeelding volledig weer te geven. Het instellen van deze vlag op true schakelt dit gedrag uit:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Als u niet van plan bent de klik op de afbeelding zelf af te handelen (zie [onImageClicked](#callbacks)), raden we aan dit te combineren met wat styling
om te voorkomen dat het lijkt alsof de afbeelding aangeklikt kan worden.

---