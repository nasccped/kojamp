# our command/flag variables
CompileCommand = javac
SpecifyDirFlag = -d
RunCommand     = java
GoToDirFlag    = --class-path



# our directories/subdirectories/files variables
SourceDir       = ./src
OutputDir       = ./out
SourceFiles     = $(wildcard $(SourceDir)/*.java)
ExpectedOutputs = $(patsubst $(SourceDir)/%, $(OutputDir)/%, $(patsubst %.java, %.class, $(SourceFiles)))
FinalExeName    = Main



# our escapes/strings variables
T_MAGENTA_ESCAPE = \e[1;38;5;213m
T_RED_ESCAPE     = \e[1;38;5;215m
RED_ESCAPE       = \e[1;31m
GREEN_ESCAPE     = \e[1;32m
YELLOW_ESCAPE    = \e[1;33m
RESET_ESCAPE     = \e[0m
TERMINAL_SEPARATOR = ----------------------------------------------------------------------



# all script >>> will just print the help message
all:
	@echo  "$(TERMINAL_SEPARATOR)"                                                              ; \
	printf "Are you trying to test $(T_MAGENTA_ESCAPE)Ko$(T_RED_ESCAPE)jamp$(RESET_ESCAPE)?\n\n"; \
	printf "You can type:\n"                                                                    ; \
	printf "\t$(YELLOW_ESCAPE)make build:$(RESET_ESCAPE) will build the outputs\n"              ; \
	printf "\t$(YELLOW_ESCAPE)make run:$(RESET_ESCAPE)   will execute the outputs\n\n"          ;



# build script >>> will generate the output exe (bytecode)
build: $(SourceFiles)
	@if [ ! -d $(OutputDir) ]; then \
		echo "$(TERMINAL_SEPARATOR)"                                                ; \
		printf "The output folder $(RED_ESCAPE)was not found$(RESET_ESCAPE)...\n"   ; \
		printf "We'll $(GREEN_ESCAPE)build one$(RESET_ESCAPE) for you! ^u^\n\n"     ; \
		mkdir $(OutputDir)                                                          ; \
	fi                                                                            ; \
	$(CompileCommand) $< $(SpecifyDirFlag) $(OutputDir)
	@echo "$(TERMINAL_SEPARATOR)"                                                 ; \
	printf "The program $(GREEN_ESCAPE)was built successfully$(RESET_ESCAPE)!\n\n"



# run script >>> will run the generated bytecode
run: $(ExpectedOutputs) $(OutputDir)/$(FinalExeName).class
	@echo "$(TERMINAL_SEPARATOR)"                                                                                     ; \
	printf "The program is $(GREEN_ESCAPE)ready to run: $(YELLOW_ESCAPE)$(OutputDir)/$(FinalExeName)$(RESET_ESCAPE)\n"; \
	echo "$(TERMINAL_SEPARATOR)"                                                                                      ; \
	printf "\n"                                                                                                       ; \
	$(RunCommand) $(GoToDirFlag) $(OutputDir) $(FinalExeName)
	@printf "\n"
