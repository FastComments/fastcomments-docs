[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments pozwala użytkownikom na przesyłanie obrazów. Kiedy użytkownik kliknie ten obraz, FastComments domyślnie otworzy nową kartę, aby wyświetlić obraz w pełnym rozmiarze. Ustawienie tego przełącznika na true wyłącza to zachowanie:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Jeśli nie zamierzasz samodzielnie przechwytywać kliknięć obrazów (zobacz [onImageClicked](#callbacks)), zalecamy połączenie tego ze stylem, który usunie wygląd sugerujący, że obraz jest klikalny.