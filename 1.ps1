# 启动目标程序
$process = Start-Process -FilePath "D:\WORKSPACE\RUST\learning\badapple_moon\target\release\badapple_moon.exe" -PassThru
$pid = $process.Id
Write-Output "程序已启动，PID 为: $pid"

# 检查 DTrace 工具是否存在
$dtracePath = "C:\Program Files\DTrace\dtrace.exe"  # 替换为你的 DTrace 工具路径
if (-Not (Test-Path $dtracePath)) {
    Write-Error "DTrace 工具未找到，请检查路径: $dtracePath"
    exit 1
}

# 构建 DTrace 命令
$dtraceCommand = "profile-99 /pid == $pid/ { @[ustack(100)] = count(); }"
Write-Output "运行 DTrace 命令: $dtraceCommand"

# 执行 DTrace 命令
Start-Process -FilePath $dtracePath -ArgumentList "-q -n '$dtraceCommand'" -NoNewWindow -Wait