input metauml;
beginfig(1);
    Package.Client("Client")();
    Client.info.foreColor := white;

    Package.MacroPatterns("macro-patterns")();
    Package.MacroLib("macro-lib")();
    Package.MacroTestHelpers("macro-test-helpers")();
    MacroTestHelpers.info.foreColor := (0.95, 0.95, 0.95);

    leftToRight(30)(Client, MacroPatterns, MacroLib);
    topToBottom.left(20)(MacroLib, MacroTestHelpers);

    drawObjects(Client, MacroPatterns, MacroLib, MacroTestHelpers);

    clink(associationUni)(Client, MacroPatterns);
    clink(associationUni)(MacroPatterns, MacroLib);
    link(associationUni)(pathManhattanY(MacroPatterns.s, MacroTestHelpers.w));
    link(associationUni)(pathVertical(MacroLib.s, MacroTestHelpers.top));
endfig;
end