locale-name = Polski

-org-name = OpenStax Poland

-brand-name = Adaptarr!



## Login page

login-field-email = Adres e-mail

login-field-password = Hasło

login-reset-password = Zresetuj hasło

# Variables:
# - $code (string): error code
login-error = { $code ->
    ["user:not-found"] Nie znaleziono użytkownika
    ["user:authenticate:bad-password"] Nieprawidłowe hasło
   *[other] Wystąpił nieznany błąd: { $code }
}



## Session elevation page

elevate-entering-superuser-mode =
    <p>Wchodzisz w tryb super-użytkownika</p>
    <p>Nie będziemy pytać o twoje hasło ponownie przez następne 15 minut</p>

elevate-field-password = Hasło

elevate-submit = Autoryzuj

# Variables:
# - $code (string): error code
elevate-error = { $code ->
    ["user:authenticate:bad-password"] Nieprawidłowe hasło
   *[other] Wystąpił nieznany błąd: { $code }
}



## Logout page

logout-message = <p>Został/aś wylogowany/a.</p>



## Registration page

register-field-name = Imię

register-field-password = Hasło

register-field-repeat-password = Hasło

register-submit = Zarejestruj

# Variables:
# - $code (string): error code
register-error = { $code ->
    ["user:password:bad-confirmation"] Hasła nie pasują
    ["user:register:email-changed"]
        Nie możesz zmienić adresu e-mail podczas rejestracji
    ["invitation:invalid"] Nieprawidłowy kod zaproszenia
    ["user:new:empty-name"] Imię nie może być puste
    ["user:new:empty-password"] Hasło nie może być puste
   *[other] Wystąpił nieznany błąd: { $code }
}



## Password reset page

reset-field-password = Hasło

reset-field-repeat-password = Hasło

reset-field-email = Adres e-mail

reset-message =
    <p>Prosimy wpisać swój adres e-mail i kliknąć “zresetuj hasło”. Instrukcje
    jak zresetować hasło wyślemy na podany adres.</p>

reset-message-sent = <p>Instrukcje zostały wysłane</p>

reset-submit = Zresetuj hasło

# Variables:
# - $code (string): error code
reset-error = { $code ->
    ["user:not-found"] Nieznany adres e-mail
    ["user:password:bad-confirmation"] Hasała nie pasują
    ["password:reset:invalid"] Niepoprawny kod resetowania hasła
    ["password:reset:passwords-dont-match"] Hasała nie pasują
    ["user:change-password:empty"] Hasło nie może być puste
   *[other] Wystąpił nieznany błąd { $code }
}



## Mail template

-mail-url = <a href="{ $url }" target="_blank" rel="noopener">{ $text }</a>

mail-logo-alt = Logo { -org-name }™

mail-footer = Wiadomość została wygenerowana automatycznie, prosimy na nią
    nie odpowiadać. Otrzymujesz ją, ponieważ posiadasz konto w { -brand-name }.



## Invitation email

mail-invite-subject = Zaproszenie

# Variables:
# - $url (string): registration URL
mail-invite-text =
    Zostałeś/aś zaproszony/a do dołączenia do { -brand-name }, stworzonego przez
    { -org-name } systemu do tłumaczenia książek.

    Aby zarejestrować się przejdź pod poniższy adres URL

        { $url }

mail-invite-before-button =
    Zostałeś/aś zaproszony/a do dołączenia do { -brand-name }, stworzonego przez
    { -org-name } systemu do tłumaczenia książek.

    Aby zarejestrować się przejdź pod poniższy adres URL

mail-invite-register-button = Zarejestruj się

mail-invite-after-button =
    Albo skopiuj poniższy URL do paska przeglądarki:
    { -mail-url(url: $url, text: $url) }

mail-invite-footer = Powyższe zaproszenie dla { $email } do dołączenia do
    aplikacji { -brand-name } zostało wysłane przez członka
    zespołu { -org-name }.



## Password reset email

mail-reset-subject = Odzyskiwanie hasła

# Variables:
# - $username (string): user's name
# - $url (string): password reset URL
mail-reset-text =
    Cześć, { $username }.

    Aby zresetować hasło przejdź pod poniższy URL

        { $url }

    Jeżeli nie prosiłeś/aś o zresetowania hasła nie masz się czym martwić,
    twoje konto jest bezpieczne.

# Variables:
# - $username (string): user's name
mail-reset-before-button =
    Cześć, { $username }

    Aby zresetować swoje hasło kliknij poniższy guzik

mail-reset-button = Zresetuj hasło

# Variables:
# - $url (string): password reset URL
mail-reset-after-button =
    Albo skopiuj poniższy URL do paska przeglądarki
    { -mail-url(url: $url, text: $url) }

    Jeżeli nie prosiłeś/aś o zresetowania hasła nie masz się czym martwić,
    twoje konto jest bezpieczne.



## Notification email
#
# Notification emails are divided into section. Each section begins with
# mail-notify-group-header-KIND, where KIND is the type of events in this
# section. Each section then contains a list of events, formatted with
# mail-notify-event-KIND.

mail-notify-subject = Powiadomienie o postępie prac

mail-notify-footer =
    Dziękujemy za udział w naszym projekcie.

    Pozdrawiamy, 
    Zespół { -org-name }

# Header displayed before notifications about module assignment.
mail-notify-group-header-assigned =
    Informacja o przydziale modułów do pracy:

# Notification about a module being assigned to a user.
#
# Variables:
# - $actorname (string): name of the user who assigned the module
# - $actorurl (string): URL to profile of the user who assigned the module
# - $moduletitle (string): title of the module which was assigned
# - $moduleurl (string): URL to the module which was assigned
# - $bookcount (number): Number of books in which the module is used
# - $booktitle (string): Title of one of books in which the module is used
# - $bookurl (string): URL to the book $booktitle
mail-notify-event-assigned-text =
    Moduł „{ $moduletitle }” ({ $moduleurl
    }) zostaje przekazany przez użytkownika { $actorname
    } do wykonania prac. { $bookcount ->
        [0] Moduł nie jest wykorzystywany w żadnej książce.
        [1] Moduł jest wykorzystywany w książce „{ $booktitle }” ({ $bookurl }).
       *[other] Moduł jest wykorzystywany w { $bookcount } książkach, w tym w „{
            $booktitle }” ({ $bookurl }).
    }
mail-notify-event-assigned =
    Moduł {
        -mail-url(url: $moduleurl, text: JOIN("„", $moduletitle, "”"))
    } zostaje przekazany przez użytkownika {
        -mail-url(url: $actorurl, text: $actorname)
    } do wykonania prac. { $bookcount ->
        [0] Moduł nie jest wykorzystywany w żadnej książce.
        [1] Moduł jest wykorzystywany w książce {
            -mail-url(url: $bookurl, text: $booktitle) }.
       *[other] Moduł jest wykorzystywany w { $bookcount } książkach, w tym w {
            -mail-url(url: $bookurl, text: $booktitle) }.
    }

-mail-notify-unknown-text =
    Możesz zapoznać się z { $count ->
        [1] nim
       *[other] nimi
    } w centrum powiadomień ({ $url }).
-mail-notify-unknown =
    Możesz zapoznać się z { $count ->
        [1] nim
       *[other] nimi
    } w { -mail-url(url: $url, text: "centrum powiadomień") }.

# Message displayed at the end of the email if in there were unknown
# notifications in addition to normal notifications.
#
# Variables:
# - $count (number): Number of unknown notifications
# - $notification_centre_url (string): URL of the notifications centre
mail-notify-also-unknown-events-text =
    Oraz { $count ->
        [1] jedno inne zdarzenie którego
        [few] { $count} inne zdarzenia których
       *[many] { $count } innych zdarzeń których
    } nie jesteśmy w stanie przedstawić w wiadomości e-mail. {
        -mail-notify-unknown-text(count: $count, url: $notification_centre_url) }
mail-notify-also-unknown-events =
    Oraz { $count ->
        [1] jedno inne zdarzenie którego
        [few] { $count} inne zdarzenia których
       *[many] { $count } innych zdarzeń których
    } nie jesteśmy w stanie przedstawić w wiadomości e-mail.
    { -mail-notify-unknown(count: $count, url: $notification_centre_url) }

# Message displayed at the end of the email if in there were only unknown
# notifications.
#
# Variables:
# - $count (number): Number of unknown notifications
# - $notification_centre_url (string): URL of the notifications centre
mail-notify-only-unknown-events-text =
    Chcemy Cię poinformować o { $count ->
        [1] jednym zdarzeniu którego
       *[other] { $count } zdarzeniach których
    } nie jesteśmy w stanie przedstawić w wiadomości e-mail. {
        -mail-notify-unknown-text(count: $count, url: $notification_centre_url) }
mail-notify-only-unknown-events =
    Chcemy Cię poinformować o { $count ->
        [1] jednym zdarzeniu którego
       *[other] { $count } zdarzeniach których
    } nie jesteśmy w stanie przedstawić w wiadomości e-mail.
    { -mail-notify-unknown(count: $count, url: $notification_centre_url) }
