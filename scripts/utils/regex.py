"""
Utils for regex operations
"""
import re

def pattern_in_str_sentence(
    pattern: str,
    sentence: str,
    target_index: int = 0
) -> str | None:
    """
    Search in a str sentence for a given pattern and return the match
    case at the `target_index`. Will return None if no matches found.

    Can still raise an error when `target_index` is greater than the
    `MatchesGroup` list length.
    """
    return None \
        if not (result := re.search(pattern, sentence)) \
        else result.group(target_index)

def pattern_in_liststr_sentence(
    pattern: str,
    sentence: list[str],
    target_index: int = 0
) -> str | None:
    """
    Take a list of str's, convert it into a single str and passes it
    to the `pattern_in_str_sentence`. Consider reading this function
    help/doc.
    """
    return pattern_in_str_sentence(pattern, "\n".join(sentence), target_index)
