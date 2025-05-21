from colors import RED_NONE, RESET_ESCAPE, GREEN_NONE
from utils.regex import patternlist_in_str

class ProgramVersion:
    """
    Store program versions (<PREFIX>xx.yy.zz) as objects and implements obj
    features (version comparing + pretty printing with json)
    """

    def __init__(self, value: str) -> None:
        # prepare sample schema (<PREFIX>xx.yy.zz)
        pat = r"([a-zA-Z]*\d+\.\d+\.\d+)"
        match_list = patternlist_in_str(pat, value)

        # a single matching is expected
        if len(match_list) != 1:
            could_not_match = RED_NONE + "Couldn't match" + RESET_ESCAPE
            value_error = "ValueError"
            print(f"{could_not_match} a ProgramVersion from the '{value}' value.")
            print(f"Raising a {value_error} exception.")
            raise ValueError()

        # separate prefix and values
        value = match_list[0]
        pat = r"([a-zA-Z]*)(\d+\.\d+\.\d+)"
        pref_matching = patternlist_in_str(pat, value)[0]

        # if len is 2 (prefix exists)
        if len(pref_matching) == 2:
            prefix = pref_matching[0]
            vers = pref_matching[1]
        else:
            prefix = None
            vers = pref_matching[0]

        # separate each version field into parsed values
        maj, min, pat = [int(v) for v in vers.split(".")]

        # set all values to inner attributes
        self.value = value
        self.prefix = prefix
        self.major = maj
        self.minor = min
        self.patch = pat

    def __lt__(self, other) -> bool:
        """
        Comparing (less than) between `ProgramVersion` objects.
        """
        # alert + raise error if incompatible objects
        if not isinstance(other, ProgramVersion):
            self_class = GREEN_NONE + self.__class__.__name__ + RESET_ESCAPE
            other_class = RED_NONE + other.__class__.__name__ + RESET_ESCAPE
            print(f"Passing invalid comparable object to `{self_class}`: `{other_class}`")
            raise ValueError()

        # separate each field
        s = [self.major, self.minor, self.patch]
        o = [other.major, other.minor, other.patch]

        return (s[0], s[1], s[2]) < (o[0], o[1], o[2])

    def __eq__(self, other) -> bool:
        """
        Comparing (equals) between `ProgramVersion` objects.
        """
        # alert + raise error if incompatible objects
        if not isinstance(other, ProgramVersion):
            self_class = GREEN_NONE + self.__class__.__name__ + RESET_ESCAPE
            other_class = RED_NONE + other.__class__.__name__ + RESET_ESCAPE
            print(f"Passing invalid comparable object to `{self_class}`: `{other_class}`")
            raise ValueError()

        # separate each field
        s = [self.major, self.minor, self.patch]
        o = [other.major, other.minor, other.patch]

        return (s[0], s[1], s[2]) == (o[0], o[1], o[2])

    def __le__(self, other) -> bool:
        """
        Comparing (less equals) between `ProgramVersion` objects.
        """
        # alert + raise error if incompatible objects
        if not isinstance(other, ProgramVersion):
            self_class = GREEN_NONE + self.__class__.__name__ + RESET_ESCAPE
            other_class = RED_NONE + other.__class__.__name__ + RESET_ESCAPE
            print(f"Passing invalid comparable object to `{self_class}`: `{other_class}`")
            raise ValueError()

        # separate each field
        s = [self.major, self.minor, self.patch]
        o = [other.major, other.minor, other.patch]

        return (s[0], s[1], s[2]) <= (o[0], o[1], o[2])

    def __str__(self, outher_class: str | None = None) -> str:
        result = "ProgramVersion"
        if outher_class:
            result += f"<{outher_class}>"
        result += f"({self.value})"
        return result
