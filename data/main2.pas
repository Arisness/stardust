Program test;

var
    x: integer;

function sum(x: integer; y: integer): integer;
begin
    { sum := sum(sum(3, 2), 1) + true; }
    x := 3;
    { sum := true <> false = true >= true; }

    sum := 2;
end;

var
    w: integer;
    z : integer;
begin
    {z := 4;}

    if 2 < sum(3, 2) then
        begin
        x := 4;
        z:='hola';
        end
    else
        begin

        x := 3;
        end;

    x := sum(5, 10);  { Example call to the function }
    writeln('The sum is: ', x);
end.