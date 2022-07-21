#!/usr/bin/env python3

SOURCE_WORDLIST_NAME = "oldlist.txt"
REJECTED_WORDS_NAME = "rejects.txt"
NEW_WORDS_NAME = "newlist.txt"

def main():
    from easy_user_input.eui import inputYesNo
    alreadyAccepted = getAcceptedWords()
    alreadyRejected = getRejectWords()
    with open(neighborFilepath(SOURCE_WORDLIST_NAME), "r") as inFile:
        with open(neighborFilepath(REJECTED_WORDS_NAME), "a") as rejectFile:
            with open(neighborFilepath(NEW_WORDS_NAME), "a") as outFile:
                for srcWord in inFile.readlines():
                    if srcWord not in alreadyAccepted and srcWord not in alreadyRejected:
                        if len(srcWord.strip().removesuffix("'s")) > 3:
                            outFile.write(srcWord)
                            alreadyAccepted.append(srcWord)
                            continue
                        relatedAccepted = findRelatedWords(srcWord, alreadyAccepted)
                        relatedRejected = findRelatedWords(srcWord, alreadyRejected)
                        if len(relatedAccepted) > 0:
                            print(f"related words accepted: {relatedAccepted}")
                        if len(relatedRejected) > 0:
                            print(f"related words rejected: {relatedRejected}")
                        if inputYesNo(f"accept \"{srcWord.strip()}\"?", True):
                            outFile.write(srcWord)
                            alreadyAccepted.append(srcWord)
                        else:
                            rejectFile.write(srcWord)
                            alreadyRejected.append(srcWord)

#returns a list of words from wordlist that may be related to target
#i.e. that are a substring of target or have target as a substring
def findRelatedWords(target, wordlist):
    target = target.strip().lower()
    output = []
    for word in filter(lambda x: len(x) > 2, map(lambda x: x.strip(), wordlist)):
        lowerword = word.lower()
        if target in lowerword or lowerword in target:
            output.append(word)
    return output


# Returns list of already rejected words
def getRejectWords():
    filepath = neighborFilepath(REJECTED_WORDS_NAME)
    try:
        with open(filepath, "r") as inFile:
            return list(inFile.readlines())
    except FileNotFoundError:
        #just return empty list
        return []
    
# Returns list of already accepted words
def getAcceptedWords():
    filepath = neighborFilepath(NEW_WORDS_NAME)
    try:
        with open(neighborFilepath(NEW_WORDS_NAME), "r") as inFile:
            return list(inFile.readlines())
    except FileNotFoundError:
        return []

# Returns path of the directory this file resides in
def currentDirPath() -> str:
    from os import path
    return path.split(__file__)[0]

# Returns the path to a file in the same directory as this file, given its name
def neighborFilepath(filename: str) -> str:
    from os import path
    return path.join(currentDirPath(), filename)

if __name__ == "__main__":
    main()