namespace wavewatch_server;

public interface IAnalyzerClient
{
    Task ReceiveTraceData(string analyzerId, TraceData traceData);
}