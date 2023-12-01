def readline(string:str) -> list[str]:
    str_list:list[str] = string.split(" ")
    new_list:list[str] = []
    for i in range(len(str_list)):
        new_list.append(str_list[i][1:-1])
    for i in range(len(new_list) - 3):
        if new_list[i] == "":
            new_list[i] = "nope" 
    return new_list


test = readline(row)
print(test)
