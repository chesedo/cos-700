input metauml;
beginfig(1);
    % Regular code flow
    Begin.StartReg;
    Activity.Reg("Program");
    End.EndReg;

    % Metaprogram flow
    Begin.StartMeta;
    Activity.Meta("Metaprogram");
    End.EndMeta;

    % Alignments
    leftToRight(80)(StartReg, Reg, EndReg);
    leftToRight(80)(StartMeta, Meta, EndMeta);
    topToBottom.midx(20)(Reg, Meta);

    % Draw objects
    drawObjects(StartReg, Reg, EndReg, StartMeta, Meta, EndMeta);

    % Links for regular
    clink(associationUni)(StartReg, Reg);
    clink(associationUni)(Reg, EndReg);

    item.InputReg(iAssoc)("user input")(InputReg.s = .5[StartReg.e, Reg.w]);
    item.OutputReg(iAssoc)("calculated output")(OutputReg.s = .5[Reg.e, EndReg.w]);

    % Links for meta
    clink(associationUni)(StartMeta, Meta);
    clink(associationUni)(Meta, EndMeta);

    item.InputMeta(iAssoc)("code input")(InputMeta.s = .5[StartMeta.e, Meta.w]);
    item.OutputMeta(iAssoc)("program")(OutputMeta.s = .5[Meta.e, EndMeta.w]);

    link(associationUni)(pathStepY(OutputMeta.n, Reg.s, 8));
endfig;
end