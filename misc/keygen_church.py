import base64

# Decodes the mystery text on the Bandcamp page for Nel Nome Del Codice by
# KEYGEN CHURCH.
#
# See: https://keygenchurch.bandcamp.com/album/nel-nome-del-codice
def nel_nome_del_codice():
    encoded = "bRt5fhw7XFwsa+9wEqtYRegCcM5ns9uw8rtIv6N6DT23RffFBH54dey/E6McUx6gDqCXyL6CG8YqWe7ref4X8MMa4qu2Jn+VEQxSemQcTKusr/vpdJGM62fRCggK8hbRodF1XteNmVOolw+heSJIEG+BiEO8TuksqXWPu1exgPVNgj+VKKC3fXdydVQ4gIybkgXtO1+wvUD242GfWndrT2/3Vsiwp9UaTqzmXt1AcMA= "
    decoded = base64.b64decode(encoded)
    return decoded.hex().upper()

# Decodes the mystery text on the Bandcamp page for Tenebre Rosso Sangue by
# KEYGEN CHURCH.
#
# See: https://keygenchurch.bandcamp.com/album/tenebre-rosso-sangue
def tenebre_rosso_sangue():
    decimal_chars = """
    070 111 108 108 111 119 032 109 101 032 105 110 116 111 032 116 104 101 032 100
    097 114 107 110 101 115 115 013 010 119 097 108 107 105 110 103 032 098 121 032
    116 104 101 032 115 105 110 110 101 114 115 032 107 110 101 101 108 105 110 103
    013 010 105 110 032 116 104 101 032 115 105 120 116 104 032 105 110 102 101 114
    110 097 108 032 099 105 114 099 108 101 013 010 119 104 101 114 101 032 116 104
    101 105 114 032 119 111 117 110 100 115 032 097 114 101 032 110 101 118 101 114
    032 104 101 097 108 105 110 103 013 010 013 010 087 101 032 100 101 115 099 101
    110 100 032 097 099 114 111 115 115 032 116 104 101 105 114 032 112 121 114 101
    013 010 098 117 114 110 105 110 103 032 116 111 109 098 115 032 111 102 032 101
    116 101 114 110 097 108 032 102 105 114 101 013 010 109 097 107 105 110 103 032
    121 111 117 114 032 119 097 121 032 111 117 116 032 098 121 032 107 105 108 108
    105 110 103 013 010 115 104 111 111 116 105 110 103 032 112 117 110 099 104 105
    110 103 032 097 110 100 032 098 108 111 111 100 032 115 112 105 108 108 105 110
    103 013 010 013 010 066 117 116 032 098 101 119 097 114 101 032 116 104 101 032
    115 111 117 110 100 032 121 111 117 032 097 114 101 032 104 101 097 114 105 110
    103 013 010 111 117 116 032 116 104 101 032 112 105 112 101 115 032 111 102 032
    111 114 103 097 110 115 032 115 099 114 101 097 109 105 110 103 013 010 104 111
    108 100 115 032 097 032 098 108 101 115 115 101 100 032 099 111 100 101 032 116
    104 097 116 039 115 032 108 101 097 100 105 110 103 013 010 116 111 032 116 104
    101 032 099 111 118 101 032 111 102 032 116 104 101 032 117 110 102 111 114 103
    105 118 105 110 103 013 010 013 010 080 097 105 110 116 101 100 032 114 101 100
    032 111 114 032 112 097 105 110 116 101 100 032 098 108 097 099 107 013 010 097
    114 101 032 116 104 101 032 099 111 108 117 109 110 115 032 121 111 117 032 115
    104 111 117 108 100 032 099 114 097 099 107 013 010 109 097 100 101 032 111 102
    032 104 105 100 100 101 110 032 108 097 121 101 114 101 100 032 115 107 105 110
    013 010 104 111 108 100 105 110 103 032 115 097 099 114 101 100 032 099 111 100
    101 032 119 105 116 104 105 110 032 013 010 013 010 076 111 111 107 032 097 114
    111 117 110 100 032 116 104 101 114 101 039 115 032 115 111 109 101 116 104 105
    110 103 032 111 100 100 013 010 119 101 105 103 104 032 121 111 117 114 032 099
    111 100 101 032 098 101 099 097 117 115 101 032 105 116 039 115 032 119 114 111
    110 103 013 010 112 117 109 112 032 105 116 032 117 112 032 097 110 100 032 121
    111 117 032 119 105 108 108 032 102 105 110 100 013 010 097 108 108 032 116 104
    101 032 100 097 116 097 032 098 121 116 101 115 032 101 110 116 119 105 110 101
    100 013 010 013 010 078 111 119 032 100 111 110 039 116 032 102 111 111 108 032
    097 114 111 117 110 100 032 097 110 100 032 115 105 110 103 013 010 116 104 101
    114 101 039 115 032 097 032 115 112 101 099 116 114 101 032 105 110 032 116 104
    101 032 115 119 105 110 103 013 010 115 099 097 110 110 105 110 103 032 115 108
    111 119 032 102 114 111 109 032 111 117 116 101 114 032 115 112 097 099 101 013
    010 100 114 097 119 105 110 103 032 115 105 103 110 097 108 115 044 032 108 101
    097 118 105 110 103 032 116 114 097 099 101 013 010 013 010 079 110 099 101 032
    121 111 117 032 107 110 111 119 032 116 104 101 032 119 111 114 100 032 116 111
    032 115 097 121 013 010 089 111 117 032 109 097 121 032 112 117 115 104 032 116
    104 101 032 100 111 111 114 032 097 115 119 097 121 013 010 066 117 116 032 098
    101 102 111 114 101 032 121 111 117 032 115 116 097 114 116 032 116 111 032 098
    108 097 109 101 013 010 073 110 115 101 114 116 032 099 111 105 110 032 116 111
    032 114 117 110 032 116 104 101 032 103 097 109 101
    """

    return ''.join(map(lambda encoded_char: chr(int(encoded_char)), decimal_chars.split()))

def third():
    keys = [
        "A8D9",
        "D1C2H9I1D9J9H3E0",
        "H0C3J4D8D5",
        "F9C1D9G5G0J2A1C1",
        "J0B1D9J4C3D9B0"
    ]
    text = """
    ░he█ ░ t░█ug█t ░ l█s░ █░ wa░
    a░d t█░ vo░█ tu░n█d ░n█░ p█i░
    T░ th░ un░██wn ░█v░ s█░ sa█░
    F█░w░rd ░█l th░ n█░ht █n░ d░█

    █ea░░hin█ ░o█ ░ c░█e ░o d░r█
    Th█░ █o░eve█ ░░f█ ░ ░░█k
    ░n m░ █o█░ a█ou░ ░░ █ra░█
    █o█ ░ █░are█ ░░ ░om█ ba░░

    ░ ░e█ s░░l█ fro█ an░ie█░ █ei░ns
    Th░█ i░ b█ea░░ng a░░ ░he █hai██
    Ne░ d░░e█░ion░ w░er█ █o ██in░
    █l░ █░e di░██s ░ c░█ p░a░

    ░oi█ ░e █░w i░t█ th█░ m█░░
    ░it░a█ m░de █░ z░█o █n░ o░e█
    Pr█░r█░░ed co█e to █a░se █ou░ ░ear░
    M█░ th░ █el░b░a░█on ░█a░█
    """
    encoded = "TQjR54nGyFNMUC5vEXgXsOcOQ7MsO8d0NKPMHckfwdO/TLCtXoWk4rjdUIm06KrCUgUnesufnmZ26NL2dkGnXH4sUXdzw7NQgGchClGkTZO8EmGSvT2zQw/mQ7Q3a21n/UajyT693x7QAGzfOOql9EforTToSz5JoDeZMI5Zp7+TjTnPtWYFSmawzBDnTLWHPjhV4dbqx0bzJhk4D+iJkRWrVeYcLgJe/jpHj/oeqb8g9O0qRwgH4qhU6zQSsPicZnRzxyc9PfF7MwhBNKon0I+TFJrlfGXL1J7ZcYON46WQp4Uk7h9ZC+wQJMfKj5PW90x1AIVtHXWgy4XsHbVylyH1WtVEDgEgeVROZ3UEPrTh6bFgwOvmsb4GNRPuMKQbyLjajLUpa5bYYpc1Pd4k16iBzWniPjhGN3You3d/ym9R9By1hVUZj+uHWSDB023BbloaCpVHHcoDyBKM7bBkkjEveDVf9tI9HcLjofhycHkm/bYMXI61/wF8dv4wJdXVM/GiZVUWpjGeoCUnERbi7zD4ttxZ4sNGnt9aMmHtlVg6cXcRIU6mfzBTlz3dz/pxbVK/ROBca1XgynyH3uix6Zu55M7ZdAFaF0zsdnAH9w4/h3qZx0EEzORWC7o9CUjWaJrv3xoJTvaNv+QoGoFAGY2OTSLerUvAAdbHt3H6mI6HY6RoumjcDVAgS74/EFrCe2ZUCJWUBSRs7ucqVYuq8Zt3tisX8MhLak1QOpEdpMnU7/Bov2LPTmYp4j5SNaaveibE4kghIiFtzENHmT9dcqd8CPPyOcL0Yxa8aFI+uL56NgpfyIdPTLrAcNiSE+NvdC/BOSeNkSyZwFRZFKslEZ8pUuKGw2CJWgbGLdI0Azlg1fTwXMczGXPjuOmqsQ4iyU/pg21l8vxSAh64BrQrxZy1jQptzhcl1iqeXA6bDUxaq/Ik+OLcR6QOfXAyPm0y"
    decoded = base64.b64decode(encoded)
    return decoded.hex().upper()

def second():
    key = "E1A5-N2T3-S6M7-C4B8"
    title = "Bullug Gegbug Ibgabiug Gixcure Dagabciea Fuic"
    encoded = "NOz8VK0pVjaREZVdL09ngv7DEoNesvT4EfERAO7W40eIYIbadzhz0O0xHFRyGWlvwi/eNR4RqazVKWcnQp4gUiCTE0RwUoZ31GZCvQ0itU5s3DERVgUEAlzkgXVxGfv7C/yoFlLrJiFbubMKM4S9jf3V1Z4gThluGHTNMBf+LkVw9az4OIFU7vgb6gIbSSX9Q10r0yon3H7qUPXCLOOycqGTh7lWk7a5COxTk7prA6Tdi5MTG8b19A3SS7WL8Zkd45JigS2PZVHVaaJmM/jKWKEfmqeieSO1jKHq6Dh9bBiV/fBaAKlgs8dC3H9iqaDhqXjLs3RdYDCiupqZgcrdXbEHq9EqwVO1c+ODORCDPDr4bTMhocVlvZwFIIl1LQ3ob8GlcDhGJWoF/R06zr3gBWKYxdSh5AQqTmeObj7rsZeZ/uYyY2VcpcuK2aLl0Mp+CDho+VN3L5PnXgS/p21y/C45MduJN8JoGGBBww5G9MB1rMkAmE4hAAd8wI0KTAcAhaoycIdTNLrw4c3RBANxaJ1mIykdl1Z86Zptj6ZOzfx014HVvAP74IEO+qR+r2SKgvrM9M+5/UFyQL22ehywi4zqHqPC/WR9N4bCRAO/F7D/K7OuFO2PYyBH+jBNKVzMf0lVtIukK5ToPlBOSd0fixqWMKSSmkUUQ5WPfkR3Pwvd164SAoXGiozUrZTa2HxFFZ7U0sZRrMp+AfcZqjKJDFsbPpNnsEyjggbtDPQF3dURnsKdLQVqLwYRSeN5BeAhbO7GD2I2IRni8wI+/GfauQ=="
    decoded = base64.b64decode(encoded)
    return decoded.hex().upper()

def first():
    key = "E1A5-N2T3-S6M7-C4B8"
    titles = [
        "Duhicebdo ed varicecag parvabiguf",
        "Igparera fihi gexigug igpariug afc",
        "E puro fobca dafluic eque pure",
        "Ohfaquiug egidof, varicef odiug peric",
        "Eliabe vicie ib odulif hehaguf e cargo bofcre fubc",
        "Baqua irefdi, baqua edgireri, fad ibcalligara",
        "Becureg axpallef furde. Cegab ufqua radurrac",
        "Baqua ighallag farodaf progabarebc equilea dolugheg",
        "Lax ubivarfe afc, quea iuhac befdi ac gori"
    ]

def main():
    print("Nel Nome Del Codice")
    print(nel_nome_del_codice())

    print("Tenebre Rosso Sangue")
    print(tenebre_rosso_sangue())

    print("░█░█░░█░█░█░")
    print(third())

    print("█ ▓")
    print(second())

    print("░ ▒ ▓ █")
    print(first())

if __name__ == '__main__':
    main()
