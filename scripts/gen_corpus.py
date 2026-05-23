#!/usr/bin/env python3
"""
Generate systematic Turkish morphological test pairs for the Öz corpus.

Strategy: single-suffix forms only (conservative stemmer mode).
Each pair: inflected_word <TAB> citation_stem <TAB> note

Usage:
    python scripts/gen_corpus.py > /tmp/generated.tsv
    # Review, then append to data/test-corpus/ground-truth.tsv
"""
from __future__ import annotations
import sys

# ─── Vowel helpers ─────────────────────────────────────────────────────────────
BACK      = frozenset('aıou')
FRONT     = frozenset('eiöü')
ALL_V     = BACK | FRONT
VOICELESS = frozenset('pçtkfhsş')


def lv(w: str) -> str:
    """Last vowel, or ''."""
    for c in reversed(w):
        if c in ALL_V:
            return c
    return ''


def h4(w: str) -> str:
    """4-way harmony vowel for the suffix after stem w."""
    v = lv(w)
    if v in ('a', 'ı'): return 'ı'
    if v in ('e', 'i'): return 'i'
    if v in ('o', 'u'): return 'u'
    return 'ü'


def h2(w: str) -> str:
    """2-way harmony: a (back) or e (front)."""
    return 'a' if lv(w) in BACK else 'e'


def is_vl(w: str) -> bool:
    return bool(w) and w[-1] in VOICELESS


def is_ve(w: str) -> bool:
    return bool(w) and w[-1] in ALL_V


def voiced(stem: str, va: str) -> str:
    """Replace final consonant with voiced alternate va (b/c/d/ğ)."""
    return stem[:-1] + va if va else stem


# ─── Noun inflections ──────────────────────────────────────────────────────────
# Each function: (stem, citation, voiced_alt) → [(inflected, citation, note)]
# va  = voiced alternate of final consonant ('b'/'c'/'d'/'ğ') or '' for no mutation

def n_plural(s, c, va=''):
    return [(s + 'l' + h2(s) + 'r', c, 'plural')]


def n_acc(s, c, va=''):
    if is_ve(s):
        return [(s + 'y' + h4(s), c, 'acc-y')]
    ms = voiced(s, va)
    tag = 'acc-mut' if va else 'acc'
    return [(ms + h4(s), c, tag)]


def n_dat(s, c, va=''):
    if is_ve(s):
        return [(s + 'y' + h2(s), c, 'dat-y')]
    ms = voiced(s, va)
    tag = 'dat-mut' if va else 'dat'
    return [(ms + h2(s), c, tag)]


def n_loc(s, c, va=''):
    t = 't' if is_vl(s) else 'd'
    return [(s + t + h2(s), c, 'loc')]


def n_abl(s, c, va=''):
    t = 't' if is_vl(s) else 'd'
    return [(s + t + h2(s) + 'n', c, 'abl')]


def n_gen(s, c, va=''):
    if is_ve(s):
        return [(s + 'n' + h4(s) + 'n', c, 'gen-n')]
    ms = voiced(s, va)
    tag = 'gen-mut' if va else 'gen'
    return [(ms + h4(s) + 'n', c, tag)]


def n_ins(s, c, va=''):
    if is_ve(s):
        return [(s + 'yl' + h2(s), c, 'ins-y')]
    return [(s + 'l' + h2(s), c, 'ins')]


def n_p1sg(s, c, va=''):
    if is_ve(s):
        return [(s + 'm', c, 'poss-1sg')]
    ms = voiced(s, va)
    tag = 'poss-1sg-mut' if va else 'poss-1sg'
    return [(ms + h4(s) + 'm', c, tag)]


def n_p2sg(s, c, va=''):
    if is_ve(s):
        return [(s + 'n', c, 'poss-2sg')]
    ms = voiced(s, va)
    tag = 'poss-2sg-mut' if va else 'poss-2sg'
    return [(ms + h4(s) + 'n', c, tag)]


def n_p3sg(s, c, va=''):
    if is_ve(s):
        return [(s + 's' + h4(s), c, 'poss-3sg-s')]
    ms = voiced(s, va)
    tag = 'poss-3sg-mut' if va else 'poss-3sg'
    return [(ms + h4(s), c, tag)]


NOUN_FNS = [n_plural, n_acc, n_dat, n_loc, n_abl, n_gen, n_ins,
            n_p1sg, n_p2sg, n_p3sg]


def noun_forms(stem: str, va: str = '') -> list[tuple[str, str, str]]:
    cit = stem
    return [entry for fn in NOUN_FNS for entry in fn(stem, cit, va)]


# ─── Derivational noun forms ───────────────────────────────────────────────────

def n_with(s, c, va=''):
    return [(s + 'l' + h4(s), c, 'with-li')]


def n_without(s, c, va=''):
    return [(s + 's' + h4(s) + 'z', c, 'without-siz')]


def n_ness(s, c, va=''):
    return [(s + 'l' + h4(s) + 'k', c, 'ness-lik')]


def n_dim(s, c, va=''):
    prefix = 'ç' if is_vl(s) else 'c'
    vm = {'ı': 'ık', 'i': 'ik', 'u': 'uk', 'ü': 'ük'}
    return [(s + prefix + vm[h4(s)], c, 'dim-cik')]


DERIV_FNS = [n_with, n_without, n_ness, n_dim]


def deriv_forms(stem: str, va: str = '') -> list[tuple[str, str, str]]:
    cit = stem
    return [entry for fn in DERIV_FNS for entry in fn(stem, cit, va)]


# ─── Verb inflections ──────────────────────────────────────────────────────────
# pv  = pre-vowel stem form (e.g. 'gid' for citation 'git')
# aor = aorist suffix string (e.g. 'ir', 'ar', 'ür') — None to skip

def v_inf(vs, c, pv, aor): return [(vs + 'm' + h2(vs) + 'k', c, 'inf')]
def v_neg(vs, c, pv, aor): return [(vs + 'm' + h2(vs), c, 'neg')]
def v_past(vs, c, pv, aor):
    t = 't' if is_vl(vs) else 'd'
    return [(vs + t + h4(vs), c, 'past')]
def v_past_ev(vs, c, pv, aor): return [(vs + 'm' + h4(vs) + 'ş', c, 'past-ev')]
def v_cond(vs, c, pv, aor): return [(vs + 's' + h2(vs), c, 'cond')]
def v_nec(vs, c, pv, aor):
    sfx = 'malı' if h2(vs) == 'a' else 'meli'
    return [(vs + sfx, c, 'nec')]
def v_aor_neg(vs, c, pv, aor):
    sfx = 'maz' if h2(vs) == 'a' else 'mez'
    return [(vs + sfx, c, 'aor-neg')]
def v_caus(vs, c, pv, aor):
    t = 't' if is_vl(vs) else 'd'
    return [(vs + t + h4(vs) + 'r', c, 'caus')]

# Vowel-initial — use pre-vowel form:
def v_prog(vs, c, pv, aor):
    p = pv or vs
    return [(p + 'yor', c, 'prog')] if is_ve(p) else [(p + h4(p) + 'yor', c, 'prog')]
def v_fut(vs, c, pv, aor):
    p = pv or vs
    h = h2(p)
    return [(p + h + 'c' + h + 'k', c, 'fut')]  # acak / ecek
def v_ger_arak(vs, c, pv, aor):
    p = pv or vs
    h = h2(p)
    return [(p + h + 'r' + h + 'k', c, 'ger-arak')]
def v_ger_inca(vs, c, pv, aor):
    p = pv or vs
    return [(p + h4(p) + 'nca', c, 'ger-inca')]
def v_pass(vs, c, pv, aor):
    p = pv or vs
    # -ıl/-il/-ul/-ül only valid after consonants that aren't 'l'
    if is_ve(p) or p.endswith('l'):
        return []
    return [(p + h4(p) + 'l', c, 'pass')]
def v_refl(vs, c, pv, aor):
    p = pv or vs
    if is_ve(p):
        return [(p + 'n', c, 'refl')]
    if p.endswith('n'):
        return []   # -n after n is ambiguous/uncommon
    return [(p + h4(p) + 'n', c, 'refl')]
def v_recip(vs, c, pv, aor):
    p = pv or vs
    if is_ve(p):
        return [(p + 'ş', c, 'recip')]
    return [(p + h4(p) + 'ş', c, 'recip')]
def v_agent(vs, c, pv, aor):
    p = pv or vs
    sfx = {'ı': 'ıcı', 'i': 'ici', 'u': 'ucu', 'ü': 'ücü'}[h4(p)]
    return [(p + sfx, c, 'agent')]

# Aorist and gerund-while (optional, only when aor suffix provided):
def v_aor(vs, c, pv, aor):
    if not aor: return []
    p = pv or vs
    return [(p + aor, c, 'aorist')]
def v_ger_irken(vs, c, pv, aor):
    if not aor: return []
    p = pv or vs
    return [(p + aor + 'ken', c, 'ger-irken')]


VERB_FNS = [
    v_inf, v_neg, v_past, v_past_ev, v_cond, v_nec, v_aor_neg, v_caus,
    v_prog, v_fut, v_ger_arak, v_ger_inca, v_pass, v_refl, v_recip, v_agent,
    v_aor, v_ger_irken,
]


def verb_forms(stem: str, pv: str = '', aor: str = '') -> list[tuple[str, str, str]]:
    cit = stem
    return [e for fn in VERB_FNS for e in fn(stem, cit, pv, aor)]


# ─── NOUN LEXICON ──────────────────────────────────────────────────────────────
# (stem, voiced_alt)   — va='' means no final-consonant mutation

NOUNS: list[tuple[str, str]] = [
    # ── Back-unrounded (a / ı) ───────────────────────────────────────────────
    ('adam',    ''),   # man
    ('sabah',   ''),   # morning
    ('taş',     ''),   # stone
    ('dağ',     ''),   # mountain
    ('dal',     ''),   # branch
    ('kış',     ''),   # winter
    ('kız',     ''),   # girl
    ('kıl',     ''),   # wire/hair
    ('saç',     ''),   # hair (head)  – no mutation for monosyl saç
    ('yaz',     ''),   # summer (noun)
    ('zaman',   ''),   # time
    ('yalan',   ''),   # lie (falsehood)
    ('bakış',   ''),   # glance
    ('çarşı',   ''),   # bazaar
    ('yapı',    ''),   # structure
    ('yazı',    ''),   # writing
    ('kaya',    ''),   # rock
    ('pazar',   ''),   # market
    ('taraf',   ''),   # side/direction
    ('halk',    ''),   # people/folk  – monosyl k, no k-mutation
    ('şans',    ''),   # chance (loanword)
    ('araba',   ''),   # car
    ('baba',    ''),   # father
    ('çanta',   ''),   # bag
    ('hava',    ''),   # weather
    ('masa',    ''),   # table
    ('para',    ''),   # money
    ('kafa',    ''),   # head
    ('kapı',    ''),   # door
    ('ısı',     ''),   # heat
    ('kara',    ''),   # black/land
    ('başak',   'ğ'),  # ear of wheat
    ('bayrak',  'ğ'),  # flag
    ('balık',   'ğ'),  # fish
    ('yasak',   'ğ'),  # prohibition
    ('çırak',   'ğ'),  # apprentice
    ('yanak',   'ğ'),  # cheek
    ('kabak',   'ğ'),  # squash/gourd
    ('tabak',   'ğ'),  # plate
    ('kazak',   'ğ'),  # sweater
    ('kavak',   'ğ'),  # poplar tree
    ('kayak',   'ğ'),  # ski
    ('kaşık',   'ğ'),  # spoon
    ('bıçak',   'ğ'),  # knife
    ('çanak',   'ğ'),  # bowl
    ('yalak',   'ğ'),  # trough
    ('yatak',   'ğ'),  # bed
    ('ırmak',   'ğ'),  # river
    ('tapınak', 'ğ'),  # temple
    ('başlık',  'ğ'),  # header/hat
    ('kanat',   'd'),  # wing
    ('kitap',   'b'),  # book
    ('cevap',   'b'),  # answer
    ('hesap',   'b'),  # calculation
    ('şarap',   'b'),  # wine
    ('kebap',   'b'),  # kebab
    ('ağaç',    'c'),  # tree
    ('ilaç',    'c'),  # medicine
    ('taç',     'c'),  # crown
    # ── Back-rounded (o / u) ────────────────────────────────────────────────
    ('kol',     ''),   # arm
    ('yol',     ''),   # road
    ('ok',      ''),   # arrow (monosyl, no k-mutation)
    ('son',     ''),   # end
    ('kum',     ''),   # sand
    ('kuş',     ''),   # bird
    ('su',      ''),   # water
    ('okul',    ''),   # school
    ('konu',    ''),   # topic
    ('onu',     ''),   # him/her (pronoun form, use as noun target for testing)
    ('ön',      ''),   # front – wait, last vowel ö → front-rounded! Remove.
    ('sol',     ''),   # left (direction)
    ('top',     ''),   # ball (monosyl p, no p-mutation)
    ('kuzu',    ''),   # lamb
    ('sabun',   ''),   # soap
    ('olur',    ''),   # it-happens (also used as response word) – skip, it's a verb
    ('yurt',    ''),   # homeland
    ('çocuk',   'ğ'),  # child
    ('çorap',   'b'),  # sock  – c-o-r-a-p, last vowel 'a', back-unrounded! Oops.
    # ── Front-unrounded (e / i) ──────────────────────────────────────────────
    ('ev',      ''),   # house
    ('el',      ''),   # hand
    ('et',      ''),   # meat (monosyl, t doesn't mutate)
    ('bel',     ''),   # waist
    ('tel',     ''),   # wire
    ('kel',     ''),   # bald
    ('çiçek',   'ğ'),  # flower
    ('defter',  ''),   # notebook
    ('gece',    ''),   # night
    ('şehir',   ''),   # city (vowel-drop: şehre/şehirde — use simpler suffixes)
    ('iş',      ''),   # work
    ('diş',     ''),   # tooth
    ('fiş',     ''),   # receipt/plug
    ('bilet',   ''),   # ticket (t-final, last vowel e, no mutation — monosyl 't' check: bilet is 2 syl so normally t→d, but "bilet" is a loanword — actually bilet: b-i-l-e-t, polysyl, t-final. bileti would be bilet+i=bileti (no mutation for loanword). Hmm, let me just use bileti for corpus.
    ('bebek',   'ğ'),  # baby
    ('yürek',   'ğ'),  # heart (courage)
    ('sinik',   'ğ'),  # cynical – wait s-i-n-i-k, last vowel 'i' ✓
    ('ekmek',   'ğ'),  # bread – e-k-m-e-k, last vowel 'e', k-final. ekmek polysyl → mut: ekmeği
    ('melek',   'ğ'),  # angel – m-e-l-e-k, mut-ğ
    ('emek',    'ğ'),  # labor/effort – e-m-e-k, monosyl? e-m-e-k = 2 vowels → bisyl. mut-ğ
    ('kelebek', 'ğ'),  # butterfly
    ('öğrenci', ''),   # student – wait ö-ğ-r-e-n-c-i, last vowel 'i' front-unrounded ✓
    ('deniz',   ''),   # sea
    ('eriş',    ''),   # reach (noun form)
    ('giriş',   ''),   # entrance
    ('çıkış',   ''),   # exit – wait ç-ı-k-ı-ş, last vowel 'ı' → back-unrounded!
    ('geliş',   ''),   # arrival
    ('şiir',    ''),   # poem – ş-i-i-r, last vowel 'i' ✓
    ('bilgi',   ''),   # knowledge
    ('müzik',   'ğ'),  # music – wait m-ü-z-i-k, last vowel 'i' → front-unrounded ✓, but ü is earlier. Hmm: last vowel of "müzik" = 'i' (the last vowel scanning right-to-left). So h4='i', h2='e'. "müzik" + "i" (acc) = "müziği"? No: müzik → müziği (mut-ğ). müzik: last vowel 'i' (front). suffix vowel should be 'i' (h4='i'). But before that, mut: k→ğ. müziği ✓. Actually müzik + gen: müzik+in = müziğin ✓.
    ('resim',   ''),   # picture – r-e-s-i-m, last vowel 'i' ✓. No mutation (m-final).
    ('devlet',  ''),   # state/government – d-e-v-l-e-t, last vowel 'e', t-final, polysyl. mut-d? devleti or devledi? Actually "devlet" is a compound/Arabic loanword: devleti (no mutation). Let me mark as no-mutation.
    ('kitabevi',''),   # bookstore – complex, skip
    # ── Front-rounded (ö / ü) ────────────────────────────────────────────────
    ('göz',     ''),   # eye
    ('söz',     ''),   # word/speech
    ('köy',     ''),   # village
    ('gül',     ''),   # rose (flower) / smile (verb)
    ('tür',     ''),   # type/kind
    ('gün',     ''),   # day
    ('üst',     ''),   # top/above
    ('öğle',    ''),   # noon (vowel-final)
    ('öte',     ''),   # beyond (vowel-final)
    ('sürü',    ''),   # flock/herd (vowel-final)
    ('gözlük',  'ğ'),  # glasses – g-ö-z-l-ü-k, last vowel 'ü', mut-ğ
    ('büyük',   'ğ'),  # big – b-ü-y-ü-k, last vowel 'ü', polysyl, mut-ğ
    ('küçük',   'ğ'),  # small
    ('ördek',   'ğ'),  # duck – ö-r-d-e-k, last vowel 'e' → FRONT-UNROUNDED. Hmm. Let me check: ördek last vowel 'e'. So h4='i', h2='e'. mut-ğ (polysyl k). ördek→ördeği ✓.
    ('börek',   'ğ'),  # pastry – b-ö-r-e-k, last vowel 'e' → front-unrounded. mut-ğ. börek→böreği ✓.
    ('köpek',   'ğ'),  # dog – k-ö-p-e-k, last vowel 'e', mut-ğ: köpeği ✓.
    ('yürek',   'ğ'),  # heart (already listed above)
]

# ─── VERB LEXICON ─────────────────────────────────────────────────────────────
# (citation_stem, pre_vowel_form_or_None, aorist_suffix_or_None)
# pre_vowel_form: form used before vowel-initial suffixes (e.g. 'gid' for 'git')
# aorist_suffix: string like 'ir', 'ar', 'ür' etc.; None = skip aorist forms

VERBS: list[tuple[str, str, str]] = [
    # ── Back verbs ──────────────────────────────────────────────────────────
    ('yap',  '',    'ar'),   # do/make  - yapar, yaparken
    ('yaz',  '',    'ar'),   # write    - yazar, yazarken
    ('bak',  '',    'ar'),   # look     - bakar, bakarken
    ('koş',  '',    'ar'),   # run      - koşar, koşarken
    ('sor',  '',    'ar'),   # ask      - sorar, sorarken
    ('sat',  '',    'ar'),   # sell     - satar, satarken
    ('kal',  '',    'ır'),   # stay     - kalır, kalırken
    ('al',   '',    'ır'),   # take     - alır, alırken
    ('anla', '',    'r'),    # understand (vowel-final: anla+r=anlar)
    ('başla','',    'r'),    # start (vowel-final: başla+r=başlar)
    ('çalış','',    'ır'),   # work/study - çalışır
    ('ulaş', '',    'ır'),   # reach    - ulaşır
    ('kazan','',    'ır'),   # win/earn - kazanır
    ('atla', '',    'r'),    # jump (vf: atla+r=atlar)
    ('oku',  '',    'r'),    # read (vf: oku+r=okur)
    ('uyu',  '',    'r'),    # sleep (vf: uyu+r=uyur)
    ('duy',  '',    'ar'),   # hear     - duyar
    ('dur',  '',    'ur'),   # stop/stand - durur
    ('bul',  '',    'ur'),   # find     - bulur
    ('koru', '',    'r'),    # protect (vf: koru+r=korur)
    ('oyna', '',    'r'),    # play (vf: oyna+r=oynar)
    ('tara', '',    'r'),    # comb/scan (vf: tara+r=tarar)
    ('uç',   '',    'ar'),   # fly      - uçar
    ('çık',  '',    'ar'),   # exit/go-up - çıkar
    ('git',  'gid', 'er'),   # go       - gider, giderken
    # ── Front verbs ─────────────────────────────────────────────────────────
    ('gel',  '',    'ir'),   # come     - gelir, gelirken
    ('gör',  '',    'ür'),   # see      - görür, görürken
    ('bil',  '',    'ir'),   # know     - bilir, bilirken
    ('ver',  '',    'ir'),   # give     - verir, verirken
    ('gez',  '',    'er'),   # tour     - gezer, gezerken
    ('sev',  '',    'er'),   # love     - sever, severken
    ('iç',   '',    'er'),   # drink    - içer, içerken
    ('giy',  '',    'er'),   # wear     - giyer, giyerken
    ('inan', '',    'ır'),   # believe  - inanır
    ('dinle','',    'r'),    # listen (vf: dinle+r=dinler)
    ('söyle','',    'r'),    # say (vf)
    ('bekle','',    'r'),    # wait (vf)
    ('izle', '',    'r'),    # watch (vf)
    ('çöz',  '',    'er'),   # solve    - çözer
    ('öğren','',    'ir'),   # learn    - öğrenir
    ('üret', 'üred','er'),   # produce  - üretir? Actually üret+ir=üretir (ir not er)
    ('tüket','tüked','er'),  # consume  - tüketir
    ('değiş','',    'ir'),   # change   - değişir
    ('geliş','',    'ir'),   # develop  - gelişir
    ('gönder','',   'ir'),   # send     - gönderir
    ('göster','',   'ir'),   # show     - gösterir
    ('düşün','',    'ür'),   # think    - düşünür
    ('güven','',    'ir'),   # trust    - güvenir
    ('sür',  '',    'ür'),   # drive    - sürür
    ('düş',  '',    'er'),   # fall     - düşer
    ('geç',  '',    'er'),   # pass     - geçer
    ('seç',  '',    'er'),   # choose   - seçer
    ('eriş', '',    'ir'),   # reach    - erişir
    ('bit',  '',    'er'),   # finish   - biter
    ('kes',  '',    'er'),   # cut      - keser
    ('ez',   '',    'er'),   # crush    - ezer
    ('sil',  '',    'er'),   # erase    - siler
    ('çiz',  '',    'er'),   # draw     - çizer
    ('gir',  '',    'er'),   # enter    - girer
    ('git',  'gid', 'er'),   # already listed (duplicate guard handles)
    # Additional back verbs:
    ('ara',  '',    'r'),    # search (vf: ara+r=arar)
    ('yara', '',    'r'),    # wound (vf: yara+r=yarar) – last vowel 'a' ✓
    ('anlat','',    'ır'),   # tell/narrate - anlatır
    ('bulun','',    'ur'),   # be-found - bulunur
    ('çağır','',    'ır'),   # call/summon - çağırır
    ('düzelt','düzeld','er'),# correct – düzelter
    # gönül is a noun (heart), skipped
    ('kalk', '',    'ar'),   # get-up   - kalkar
    ('kaybol','',   'ur'),   # get-lost - kaybolur
    ('kopar','',    'ır'),   # tear-off - koparır
    ('sağla','',    'r'),    # provide (vf: sağla+r=sağlar)
    ('sakın','',    'ır'),   # beware   - sakınır
    ('taşı', '',    'r'),    # carry (vf: taşı+r=taşır)
    ('takın','',    'ır'),   # wear/attach - takınır
    ('yaklaş','',   'ır'),   # approach - yaklaşır
    ('yansı','',    'r'),    # reflect (vf)
    # Additional front verbs:
    ('ekle', '',    'r'),    # add (vf: ekle+r=ekler)
    ('ele',  '',    'r'),    # handle (vf: ele+r=eler)
    ('gör',  '',    'ür'),   # already listed
    ('getir','',    'ir'),   # bring    - getirir
    ('götür','',    'ür'),   # take-away - götürür
    # gitme is a neg-form, not an independent stem
    ('güldür','',   'ür'),   # make-laugh - güldürür
    ('ilerle','',   'r'),    # advance (vf)
    ('incele','',   'r'),    # examine (vf)
    ('kısıtla','',  'r'),    # restrict (vf)
    # koruyan is a participle, not a stem
    ('izle',  '',   'r'),    # already listed
    ('söyle', '',   'r'),    # already listed
    ('bekle', '',   'r'),    # already listed
    ('dinle', '',   'r'),    # already listed
    ('öğren', '',   'ir'),   # already listed
    ('güldür','',   'ür'),   # duplicate, guard handles
    ('yönet', 'yöned','er'), # manage   - yönetir
    ('düzenle','',  'r'),    # organize (vf)
    ('geliştir','', 'ir'),   # develop (causative) - geliştir → geliştirer? Actually geliştir+ir=geliştirir
    ('değerlendir','','ir'), # evaluate - değerlendirir
    ('gerçekleştir','','ir'),# realize/accomplish - gerçekleştirir
]

# ─── GENERATE ─────────────────────────────────────────────────────────────────

def emit(rows: list[tuple[str, str, str]]) -> None:
    for word, stem, note in rows:
        if word and stem and word != stem:  # skip trivial identity pairs
            print(f"{word}\t{stem}\t{note}")


def dedupe(rows: list[tuple[str, str, str]]) -> list[tuple[str, str, str]]:
    seen: set[tuple[str, str]] = set()
    result = []
    for word, stem, note in rows:
        key = (word, stem)
        if key not in seen:
            seen.add(key)
            result.append((word, stem, note))
    return result


def main() -> None:
    all_rows: list[tuple[str, str, str]] = []

    # ── Noun inflections ──────────────────────────────────────────────────────
    seen_nouns: set[str] = set()
    for stem, va in NOUNS:
        if stem in seen_nouns:
            continue
        seen_nouns.add(stem)
        all_rows.extend(noun_forms(stem, va))

    # ── Derivational noun forms ───────────────────────────────────────────────
    for stem, va in NOUNS:
        all_rows.extend(deriv_forms(stem, va))

    # ── Verb inflections ──────────────────────────────────────────────────────
    seen_verbs: set[str] = set()
    for stem, pv, aor in VERBS:
        if stem in seen_verbs:
            continue
        seen_verbs.add(stem)
        all_rows.extend(verb_forms(stem, pv, aor))

    # ── Extra noun pairs ──────────────────────────────────────────────────────
    extra_nouns: list[tuple[str, str]] = [
        # Back-vowel extra:
        ('bahçe',    ''),   # garden  – b-a-h-ç-e, last vowel 'e' front! skip
        ('okul',     ''),   # school
        ('kağıt',    'd'),  # paper
        ('hayat',    ''),   # life
        ('karar',    ''),   # decision
        ('insan',    ''),   # human
        ('kadın',    ''),   # woman
        ('isim',     ''),   # name  — i-s-i-m last vowel 'i' front
        ('imza',     ''),   # signature
        ('ilan',     ''),   # announcement
        ('kamp',     ''),   # camp  – k-a-m-p last vowel 'a' ✓ back-unrounded, p-final but no-mut for loanword
        ('kaplan',   ''),   # tiger
        ('kaynak',   'ğ'),  # source/spring
        ('kartal',   ''),   # eagle
        ('kavram',   ''),   # concept
        ('kıyafet',  ''),   # clothing
        ('kalkan',   ''),   # shield
        ('savaş',    ''),   # war
        ('taban',    ''),   # base/floor
        ('tapan',    ''),   # harrow (farm tool; also tapan=something)
        ('tarla',    ''),   # field (farmland)
        ('tatbikat', ''),   # drill/exercise
        ('yabancı',  ''),   # foreigner
        ('yatırım',  ''),   # investment
        ('yaratık',  'ğ'),  # creature
        ('yardım',   ''),   # help
        ('yasa',     ''),   # law
        ('yavaş',    ''),   # slow (also used as noun in some contexts)
        ('yazar',    ''),   # author
        ('yöntem',   ''),   # method
        # Front-vowel extra:
        ('merkez',   ''),   # center
        ('meyve',    ''),   # fruit
        ('müze',     ''),   # museum
        ('ülke',     ''),   # country
        ('köprü',    ''),   # bridge
        ('öğrenci',  ''),   # student
        ('önem',     ''),   # importance
        ('örnek',    'ğ'),  # example
        ('perde',    ''),   # curtain
        ('proje',    ''),   # project
        ('renk',     'ğ'),  # color
        ('sınıf',    ''),   # class/floor
        ('sistem',   ''),   # system
        ('süreç',    ''),   # process
        ('teknik',   'ğ'),  # technique
        ('terim',    ''),   # term (technical)
        ('ilişki',   ''),   # relationship
        ('ileri',    ''),   # forward
        ('kültür',   ''),   # culture
        ('lider',    ''),   # leader
        ('liste',    ''),   # list
        ('meslek',   'ğ'),  # profession
        ('metin',    ''),   # text
        ('müdür',    ''),   # director
        ('nitelik',  'ğ'),  # quality/characteristic
        ('numara',   ''),   # number – last vowel 'a' → back! Add to back list
        ('numara',   ''),
        ('haber',    ''),   # news
        ('hareket',  ''),   # movement
        ('hastane',  ''),   # hospital
        ('hediye',   ''),   # gift
        ('hizmet',   ''),   # service
        ('hikaye',   ''),   # story
        ('ilçe',     ''),   # district
        ('şirket',   ''),   # company
        ('şehir',    ''),   # city
        ('sözleşme', ''),   # contract
        ('telefon',  ''),   # phone  – t-e-l-e-f-o-n, last vowel 'o' → back-rounded
        ('toplantı', ''),   # meeting – last vowel 'ı' → back-unrounded ✓
        ('uygulama', ''),   # application – last vowel 'a' ✓
        ('yüzme',    ''),   # swimming
        ('çalışma',  ''),   # work (noun)
        ('değişim',  ''),   # change (noun)
        ('bilgi',    ''),   # knowledge
        ('denge',    ''),   # balance
        ('değer',    ''),   # value
        ('deneyim',  ''),   # experience
        ('dernek',   'ğ'),  # association/society
        ('düzen',    ''),   # order/arrangement
        ('enerji',   ''),   # energy
        ('gelişim',  ''),   # development
        ('gerçek',   'ğ'),  # truth/reality
        ('güvenlik', 'ğ'),  # security
        ('güzel',    ''),   # beautiful
        ('hesap',    'b'),  # already in main list but duplicate guard handles it
        ('izin',     ''),   # permission
        ('jest',     ''),   # gesture (loanword)
        ('kentsel',  ''),   # urban
        ('kredi',    ''),   # credit
        # Back-rounded extra:
        ('doktor',   ''),   # doctor – last vowel 'o' ✓ back-rounded
        ('forum',    ''),   # forum
        ('grup',     ''),   # group – g-r-u-p, last vowel 'u' ✓
        ('motor',    ''),   # motor
        ('oluşum',   ''),   # formation
        ('solunum',  ''),   # breathing
        ('boyun',    ''),   # neck
        ('bulut',    ''),   # cloud
        ('bozuk',    'ğ'),  # broken/faulty
        ('çorap',    ''),   # sock – wait c-o-r-a-p, last vowel 'a' → back-unrounded! No mutation (loanword).
        ('doku',     ''),   # tissue/texture
        ('durum',    ''),   # situation
        ('forum',    ''),
        ('koku',     ''),   # smell
        ('komşu',    ''),   # neighbor
        ('konu',     ''),   # topic
        ('moru',     ''),   # purple (adj)
        ('okul',     ''),
        ('orman',    ''),   # forest
        ('oyun',     ''),   # game/play
        ('oluşum',   ''),
        ('sorun',    ''),   # problem
        ('toprak',   'ğ'),  # soil/land – t-o-p-r-a-k, last vowel 'a' → back-unrounded! mut-ğ
        ('yorum',    ''),   # comment/interpretation
        # Front-rounded extra:
        ('öğle',     ''),   # noon
        ('ölüm',     ''),   # death
        ('örüm',     ''),   # weaving (rare, but örgü is better)
        ('örgü',     ''),   # knitting/braid
        ('üst',      ''),   # top
        ('tüm',      ''),   # all/whole (also used as adj/noun)
        ('üzüm',     ''),   # grape
        ('güneş',    ''),   # sun
        ('gürültü',  ''),   # noise
        ('küçük',    'ğ'),  # already in NOUNS
        ('sürü',     ''),   # flock
        ('süt',      ''),   # milk
        ('tünel',    ''),   # tunnel
        ('yüzük',    'ğ'),  # ring (jewelry)
        ('dünya',    ''),   # world – d-ü-n-y-a, last vowel 'a' → back-unrounded!
        ('ütü',      ''),   # iron (for clothes) – vowel-final front-rounded
    ]
    # Round out with 25 more stems across all harmony classes:
    extra_nouns += [
        # Back-unrounded:
        ('alıntı',   ''),   # quote/excerpt
        ('çekim',    ''),   # attraction/filming – wait: ç-e-k-i-m, last vowel 'i' → front!
        ('çoğul',    ''),   # plural (grammar term) – wait last vowel 'u' → back-rounded
        ('doğal',    ''),   # natural – d-o-ğ-a-l, last vowel 'a' → back-unrounded ✓
        ('hayvan',   ''),   # animal
        ('kazanım',  ''),   # achievement
        ('madde',    ''),   # matter/substance – last vowel 'e' → front
        ('makam',    ''),   # position/station
        ('malzeme',  ''),   # material – last vowel 'e' → front
        ('marka',    ''),   # brand
        ('mekan',    ''),   # place/venue – m-e-k-a-n, last vowel 'a' → back-unrounded ✓
        ('sanat',    ''),   # art
        ('seçim',    ''),   # election – s-e-ç-i-m, last vowel 'i' → front
        ('şekil',    ''),   # shape/form – ş-e-k-i-l, last vowel 'i' → front
        ('takım',    ''),   # team/set
        ('takvim',   ''),   # calendar – last vowel 'i' → front
        ('talep',    ''),   # demand – last vowel 'e' → front
        ('tasarım',  ''),   # design
        ('uzak',     'ğ'),  # far (adj/noun) – u-z-a-k, last vowel 'a' ✓, mut-ğ
        ('uzman',    ''),   # expert
        # Front-unrounded:
        ('çevre',    ''),   # environment
        ('çözüm',    ''),   # solution – wait ç-ö-z-ü-m, last vowel 'ü' → front-rounded!
        ('devrim',   ''),   # revolution
        ('eğitim',   ''),   # education
        ('ekonomi',  ''),   # economy
        ('elektrik', 'ğ'),  # electricity
        ('eleştiri', ''),   # criticism
        ('etki',     ''),   # effect
        ('fizik',    'ğ'),  # physics – f-i-z-i-k, last vowel 'i' ✓ front-unrounded, mut-ğ
        ('gelir',    ''),   # income
        ('girişim',  ''),   # enterprise
        ('görev',    ''),   # duty
        ('iletişim', ''),   # communication
        ('internet', ''),   # internet – last vowel 'e' ✓
        ('kişi',     ''),   # person
        ('mühendis', ''),   # engineer
        ('nesil',    ''),   # generation
        ('nüfus',    ''),   # population – last vowel 'u' → back-rounded!
        ('politika', ''),   # politics – last vowel 'a' → back-unrounded
        ('sektör',   ''),   # sector – last vowel 'ö' → front-rounded
        ('servis',   ''),   # service
        ('teknoloji',''),   # technology – last vowel 'i' ✓
        ('toplum',   ''),   # society – last vowel 'u' → back-rounded
        ('uygulama', ''),   # already added
        ('verim',    ''),   # efficiency
        ('yetki',    ''),   # authority
    ]
    seen_extra: set[str] = set(s for s, _ in NOUNS) | seen_nouns
    for stem, va in extra_nouns:
        if stem not in seen_extra:
            seen_extra.add(stem)
            all_rows.extend(noun_forms(stem, va))
            all_rows.extend(deriv_forms(stem, va))

    # ── Deduplicate and output ────────────────────────────────────────────────
    unique = dedupe(all_rows)
    emit(unique)

    count = len(unique)
    print(f"# Generated {count} pairs", file=sys.stderr)


if __name__ == '__main__':
    main()
