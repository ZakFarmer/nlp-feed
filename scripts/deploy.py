from sagemaker.huggingface import HuggingFaceModel
import sagemaker

role = sagemaker.get_execution_role()

hub = {
    'HF_MODEL_ID': 'EleutherAI/gpt-neo-2.7B',
    'HF_TASK': 'text-generation'
}

huggingface_model = HuggingFaceModel(
    transformers_version='4.17.0',
    pytorch_version='1.10.2',
    py_version='py38',
    env=hub,
    role=role,
)

predictor = huggingface_model.deploy(
    initial_instance_count=1,
    instance_type='ml.t2.medium'
)
