from kolos.cli import run_sample


def test_run_sample_executes_packaged_assembly(capsys):
    run_sample()

    assert capsys.readouterr().out.strip() == "30"
